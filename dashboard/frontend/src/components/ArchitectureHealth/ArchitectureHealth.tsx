import React from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  LinearProgress,
  Grid,
  Chip,
  CircularProgress,
  Skeleton
} from '@mui/material';
import {
  TrendingUp,
  TrendingDown,
  TrendingFlat,
  ArrowUpward,
  ArrowDownward
} from '@mui/icons-material';
import { ArchitectureHealthProps } from '../../types';
import { format } from 'date-fns';

const ArchitectureHealth: React.FC<ArchitectureHealthProps> = ({
  health,
  isLoading = false,
  showTrends = true
}) => {
  if (isLoading) {
    return (
      <Card sx={{ height: '100%' }}>
        <CardContent>
          <Typography variant="h6" gutterBottom>
            Architecture Health
          </Typography>
          <Box sx={{ display: 'flex', alignItems: 'center', mb: 3 }}>
            <Skeleton variant="circular" width={80} height={80} />
            <Box sx={{ ml: 2, flex: 1 }}>
              <Skeleton variant="text" width="60%" height={24} />
              <Skeleton variant="text" width="40%" height={20} />
            </Box>
          </Box>
          <Grid container spacing={2}>
            {[1, 2, 3, 4, 5].map((i) => (
              <Grid item xs={12} key={i}>
                <Skeleton variant="rectangular" height={20} sx={{ mb: 1 }} />
                <Skeleton variant="text" width="30%" />
              </Grid>
            ))}
          </Grid>
        </CardContent>
      </Card>
    );
  }

  if (!health) {
    return (
      <Card sx={{ height: '100%' }}>
        <CardContent>
          <Typography variant="h6" gutterBottom>
            Architecture Health
          </Typography>
          <Box
            sx={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              minHeight: 200,
              color: 'text.secondary'
            }}
          >
            <Typography>No health data available</Typography>
          </Box>
        </CardContent>
      </Card>
    );
  }

  // Health score color mapping
  const getHealthColor = (score: number) => {
    if (score >= 80) return '#4CAF50'; // Green
    if (score >= 60) return '#FF9800'; // Orange
    if (score >= 40) return '#FF5722'; // Red-orange
    return '#F44336'; // Red
  };

  // Trend icon mapping
  const getTrendIcon = (direction: string, velocity: number) => {
    const color = direction === 'improving' ? '#4CAF50' : 
                 direction === 'degrading' ? '#F44336' : '#9E9E9E';
    
    switch (direction) {
      case 'improving':
        return <TrendingUp sx={{ color, fontSize: 20 }} />;
      case 'degrading':
        return <TrendingDown sx={{ color, fontSize: 20 }} />;
      default:
        return <TrendingFlat sx={{ color, fontSize: 20 }} />;
    }
  };

  // Metric progress bar with label
  const MetricBar: React.FC<{ 
    label: string; 
    value: number; 
    max?: number; 
    unit?: string;
    reverse?: boolean;
  }> = ({ label, value, max = 100, unit = '%', reverse = false }) => {
    const normalizedValue = Math.max(0, Math.min(100, (value / max) * 100));
    const displayValue = unit === '%' ? Math.round(value) : value;
    
    // For metrics where lower is better (like technical debt)
    const color = reverse ? 
      (normalizedValue <= 20 ? 'success' : 
       normalizedValue <= 50 ? 'warning' : 'error') :
      (normalizedValue >= 80 ? 'success' : 
       normalizedValue >= 60 ? 'warning' : 'error');

    return (
      <Box sx={{ mb: 2 }}>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
          <Typography variant="body2" color="text.secondary">
            {label}
          </Typography>
          <Typography variant="body2" fontWeight="medium">
            {displayValue}{unit}
          </Typography>
        </Box>
        <LinearProgress
          variant="determinate"
          value={normalizedValue}
          color={color}
          sx={{
            height: 8,
            borderRadius: 4,
            bgcolor: 'grey.200'
          }}
        />
      </Box>
    );
  };

  const healthScore = Math.round(health.overallScore);
  const healthColor = getHealthColor(healthScore);

  return (
    <Card sx={{ height: '100%' }}>
      <CardContent>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
          <Typography variant="h6">
            Architecture Health
          </Typography>
          <Typography variant="caption" color="text.secondary">
            {format(new Date(health.timestamp), 'MMM dd, HH:mm')}
          </Typography>
        </Box>

        {/* Overall Score */}
        <Box sx={{ display: 'flex', alignItems: 'center', mb: 4 }}>
          <Box sx={{ position: 'relative', display: 'inline-flex' }}>
            <CircularProgress
              variant="determinate"
              value={healthScore}
              size={80}
              thickness={4}
              sx={{
                color: healthColor,
                '& .MuiCircularProgress-circle': {
                  strokeLinecap: 'round'
                }
              }}
            />
            <Box
              sx={{
                top: 0,
                left: 0,
                bottom: 0,
                right: 0,
                position: 'absolute',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center'
              }}
            >
              <Typography variant="h6" component="div" color={healthColor} fontWeight="bold">
                {healthScore}
              </Typography>
            </Box>
          </Box>
          
          <Box sx={{ ml: 2, flex: 1 }}>
            <Typography variant="h5" gutterBottom>
              {health.repository}
            </Typography>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
              <Chip
                size="small"
                label={
                  healthScore >= 80 ? 'Excellent' :
                  healthScore >= 60 ? 'Good' :
                  healthScore >= 40 ? 'Fair' : 'Poor'
                }
                sx={{
                  bgcolor: healthColor,
                  color: 'white',
                  fontWeight: 'medium'
                }}
              />
              {showTrends && (
                <>
                  {getTrendIcon(health.trends.direction, health.trends.velocity)}
                  <Typography variant="body2" color="text.secondary">
                    {health.trends.direction} ({Math.abs(health.trends.velocity).toFixed(1)}%)
                  </Typography>
                </>
              )}
            </Box>
          </Box>
        </Box>

        {/* Detailed Metrics */}
        <Grid container spacing={2}>
          <Grid item xs={12}>
            <Typography variant="subtitle2" color="text.secondary" sx={{ mb: 2, fontWeight: 600 }}>
              DETAILED METRICS
            </Typography>
          </Grid>
          
          <Grid item xs={12}>
            <MetricBar
              label="Code Coverage"
              value={health.metrics.coverage}
              unit="%"
            />
          </Grid>

          <Grid item xs={12}>
            <MetricBar
              label="Compliance Score"
              value={health.metrics.compliance}
              unit="%"
            />
          </Grid>

          <Grid item xs={12}>
            <MetricBar
              label="Maintainability Index"
              value={health.metrics.maintainability}
              unit="%"
            />
          </Grid>

          <Grid item xs={12}>
            <MetricBar
              label="Technical Debt"
              value={health.metrics.technicalDebt}
              max={100}
              unit="%"
              reverse={true}
            />
          </Grid>

          <Grid item xs={12}>
            <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <Typography variant="body2" color="text.secondary">
                Active Drift Issues
              </Typography>
              <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                <Typography variant="body2" fontWeight="medium" color="error.main">
                  {health.metrics.driftCount}
                </Typography>
                {health.metrics.driftCount > 0 && (
                  <ArrowUpward sx={{ fontSize: 16, color: 'error.main' }} />
                )}
              </Box>
            </Box>
          </Grid>
        </Grid>

        {/* Trend Summary */}
        {showTrends && (
          <Box sx={{ mt: 3, p: 2, bgcolor: 'grey.50', borderRadius: 1 }}>
            <Typography variant="caption" color="text.secondary" sx={{ fontWeight: 600 }}>
              TREND SUMMARY
            </Typography>
            <Typography variant="body2" sx={{ mt: 0.5 }}>
              Architecture health is {' '}
              <span style={{ 
                color: health.trends.direction === 'improving' ? '#4CAF50' : 
                       health.trends.direction === 'degrading' ? '#F44336' : '#9E9E9E',
                fontWeight: 600 
              }}>
                {health.trends.direction}
              </span>
              {' '}at a rate of {Math.abs(health.trends.velocity).toFixed(1)}% per week.
              {health.trends.direction === 'degrading' && health.trends.velocity > 5 && (
                <span style={{ color: '#F44336', fontWeight: 600 }}>
                  {' '}Immediate attention recommended.
                </span>
              )}
            </Typography>
          </Box>
        )}
      </CardContent>
    </Card>
  );
};

export default ArchitectureHealth;