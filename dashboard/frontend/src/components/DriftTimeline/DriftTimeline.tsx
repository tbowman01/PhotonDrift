import React, { useEffect, useRef, useMemo } from 'react';
import * as d3 from 'd3';
import { Box, Typography, Skeleton, Tooltip, IconButton } from '@mui/material';
import { ZoomIn, ZoomOut, RestartAlt } from '@mui/icons-material';
import { DriftTimelineProps, DriftEvent } from '../../types';
import { format, parseISO } from 'date-fns';

const DriftTimeline: React.FC<DriftTimelineProps> = ({
  events,
  height = 400,
  onEventClick,
  selectedTimeRange,
  isLoading = false
}) => {
  const svgRef = useRef<SVGSVGElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  // Process and filter events based on time range
  const processedEvents = useMemo(() => {
    if (!events.length) return [];

    const now = new Date();
    let startDate: Date;

    if (selectedTimeRange.hours) {
      startDate = new Date(now.getTime() - selectedTimeRange.hours * 60 * 60 * 1000);
    } else if (selectedTimeRange.days) {
      startDate = new Date(now.getTime() - selectedTimeRange.days * 24 * 60 * 60 * 1000);
    } else if (selectedTimeRange.months) {
      startDate = new Date(now.getTime() - selectedTimeRange.months * 30 * 24 * 60 * 60 * 1000);
    } else {
      startDate = new Date(now.getTime() - 24 * 60 * 60 * 1000); // Default: last 24 hours
    }

    return events
      .filter(event => new Date(event.timestamp) >= startDate)
      .sort((a, b) => new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime());
  }, [events, selectedTimeRange]);

  // Color mapping for severity levels
  const severityColors = {
    low: '#4CAF50',
    medium: '#FF9800',
    high: '#FF5722',
    critical: '#F44336'
  };

  const createChart = () => {
    if (!svgRef.current || !containerRef.current || processedEvents.length === 0) return;

    const container = containerRef.current;
    const svg = d3.select(svgRef.current);
    
    // Clear previous chart
    svg.selectAll('*').remove();

    const margin = { top: 20, right: 80, bottom: 60, left: 60 };
    const width = container.clientWidth - margin.left - margin.right;
    const chartHeight = height - margin.top - margin.bottom;

    const g = svg
      .attr('width', container.clientWidth)
      .attr('height', height)
      .append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`);

    // Create scales
    const xScale = d3.scaleTime()
      .domain(d3.extent(processedEvents, d => new Date(d.timestamp)) as [Date, Date])
      .range([0, width]);

    // Group events by severity for stacking
    const severityLevels = ['low', 'medium', 'high', 'critical'];
    const eventsByTime = d3.rollup(
      processedEvents,
      v => {
        const counts = { low: 0, medium: 0, high: 0, critical: 0 };
        v.forEach(event => counts[event.severity]++);
        return counts;
      },
      d => d3.timeHour.floor(new Date(d.timestamp)).getTime()
    );

    // Convert to array for easier processing
    const timeData = Array.from(eventsByTime.entries()).map(([time, counts]) => ({
      time: new Date(time),
      ...counts,
      total: Object.values(counts).reduce((sum, count) => sum + count, 0)
    }));

    const yScale = d3.scaleLinear()
      .domain([0, d3.max(timeData, d => d.total) || 0])
      .range([chartHeight, 0]);

    // Create area generators for each severity level
    const stack = d3.stack<any>()
      .keys(severityLevels)
      .order(d3.stackOrderNone)
      .offset(d3.stackOffsetNone);

    const stackedData = stack(timeData);

    const area = d3.area<any>()
      .x((d: any) => xScale(d.data.time))
      .y0((d: any) => yScale(d[0]))
      .y1((d: any) => yScale(d[1]))
      .curve(d3.curveMonotoneX);

    // Add stacked areas
    g.selectAll('.severity-area')
      .data(stackedData)
      .enter()
      .append('path')
      .attr('class', 'severity-area')
      .attr('d', area)
      .style('fill', (d: any) => severityColors[d.key as keyof typeof severityColors])
      .style('opacity', 0.8)
      .style('stroke', 'white')
      .style('stroke-width', 1);

    // Add individual event points
    g.selectAll('.event-point')
      .data(processedEvents)
      .enter()
      .append('circle')
      .attr('class', 'event-point')
      .attr('cx', d => xScale(new Date(d.timestamp)))
      .attr('cy', d => {
        const timeKey = d3.timeHour.floor(new Date(d.timestamp)).getTime();
        const hourData = eventsByTime.get(timeKey);
        if (!hourData) return yScale(0);
        
        // Calculate y position based on stacking
        let yPos = 0;
        for (const severity of severityLevels) {
          if (severity === d.severity) {
            yPos += hourData[severity as keyof typeof hourData] / 2;
            break;
          }
          yPos += hourData[severity as keyof typeof hourData];
        }
        return yScale(yPos);
      })
      .attr('r', 4)
      .style('fill', d => severityColors[d.severity])
      .style('stroke', 'white')
      .style('stroke-width', 2)
      .style('cursor', onEventClick ? 'pointer' : 'default')
      .on('click', onEventClick ? (event, d) => {
        event.stopPropagation();
        onEventClick(d);
      } : null)
      .on('mouseover', function(event, d) {
        // Show tooltip
        const tooltip = d3.select('body')
          .append('div')
          .attr('class', 'chart-tooltip')
          .style('position', 'absolute')
          .style('background', 'rgba(0,0,0,0.8)')
          .style('color', 'white')
          .style('padding', '8px')
          .style('border-radius', '4px')
          .style('font-size', '12px')
          .style('pointer-events', 'none')
          .style('z-index', 1000)
          .html(`
            <strong>${d.title}</strong><br/>
            <strong>Severity:</strong> ${d.severity}<br/>
            <strong>Category:</strong> ${d.category}<br/>
            <strong>Time:</strong> ${format(new Date(d.timestamp), 'MMM dd, HH:mm')}<br/>
            <strong>File:</strong> ${d.location.file}
          `);

        tooltip
          .style('left', (event.pageX + 10) + 'px')
          .style('top', (event.pageY - 10) + 'px');

        d3.select(this)
          .transition()
          .duration(150)
          .attr('r', 6);
      })
      .on('mouseout', function() {
        d3.selectAll('.chart-tooltip').remove();
        d3.select(this)
          .transition()
          .duration(150)
          .attr('r', 4);
      });

    // Add axes
    const xAxis = d3.axisBottom(xScale)
      .tickFormat(d3.timeFormat('%H:%M') as any)
      .ticks(d3.timeHour.every(2));

    const yAxis = d3.axisLeft(yScale)
      .ticks(5)
      .tickFormat(d3.format('d'));

    g.append('g')
      .attr('class', 'x-axis')
      .attr('transform', `translate(0,${chartHeight})`)
      .call(xAxis)
      .selectAll('text')
      .style('font-size', '12px');

    g.append('g')
      .attr('class', 'y-axis')
      .call(yAxis)
      .selectAll('text')
      .style('font-size', '12px');

    // Add axis labels
    g.append('text')
      .attr('class', 'x-axis-label')
      .attr('x', width / 2)
      .attr('y', chartHeight + 45)
      .style('text-anchor', 'middle')
      .style('font-size', '14px')
      .style('fill', '#666')
      .text('Time');

    g.append('text')
      .attr('class', 'y-axis-label')
      .attr('transform', 'rotate(-90)')
      .attr('x', -chartHeight / 2)
      .attr('y', -40)
      .style('text-anchor', 'middle')
      .style('font-size', '14px')
      .style('fill', '#666')
      .text('Events per Hour');

    // Add legend
    const legend = g.append('g')
      .attr('class', 'legend')
      .attr('transform', `translate(${width + 20}, 20)`);

    severityLevels.forEach((severity, index) => {
      const legendItem = legend.append('g')
        .attr('transform', `translate(0, ${index * 25})`);

      legendItem.append('rect')
        .attr('width', 15)
        .attr('height', 15)
        .style('fill', severityColors[severity as keyof typeof severityColors]);

      legendItem.append('text')
        .attr('x', 20)
        .attr('y', 12)
        .style('font-size', '12px')
        .style('fill', '#333')
        .text(severity.charAt(0).toUpperCase() + severity.slice(1));
    });
  };

  useEffect(() => {
    if (!isLoading) {
      createChart();
    }
  }, [processedEvents, height, isLoading, onEventClick]);

  // Handle resize
  useEffect(() => {
    const handleResize = () => {
      if (!isLoading) {
        createChart();
      }
    };

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, [processedEvents, height, isLoading, onEventClick]);

  if (isLoading) {
    return (
      <Box sx={{ width: '100%', height }}>
        <Skeleton variant="rectangular" width="100%" height={height} />
      </Box>
    );
  }

  if (processedEvents.length === 0) {
    return (
      <Box
        sx={{
          width: '100%',
          height,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          border: '1px solid #e0e0e0',
          borderRadius: 1,
          bgcolor: '#f5f5f5'
        }}
      >
        <Typography variant="h6" color="textSecondary">
          No drift events found for the selected time range
        </Typography>
      </Box>
    );
  }

  return (
    <Box sx={{ width: '100%', height, position: 'relative' }}>
      <Box
        sx={{
          position: 'absolute',
          top: 8,
          right: 8,
          zIndex: 10,
          display: 'flex',
          gap: 1
        }}
      >
        <Tooltip title="Reset zoom">
          <IconButton
            size="small"
            onClick={createChart}
            sx={{ bgcolor: 'rgba(255,255,255,0.8)' }}
          >
            <RestartAlt />
          </IconButton>
        </Tooltip>
      </Box>

      <div ref={containerRef} style={{ width: '100%', height: '100%' }}>
        <svg ref={svgRef} style={{ width: '100%', height: '100%' }} />
      </div>

      <Typography
        variant="caption"
        color="textSecondary"
        sx={{ position: 'absolute', bottom: 8, left: 16 }}
      >
        Showing {processedEvents.length} events in {selectedTimeRange.label.toLowerCase()}
      </Typography>
    </Box>
  );
};

export default DriftTimeline;