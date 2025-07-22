import React from 'react';
import {
  Grid,
  Card,
  CardContent,
  Typography,
  Box,
  Chip,
  Skeleton,
  IconButton,
  Tooltip
} from '@mui/material';
import {
  BugReport,
  CheckCircle,
  Assessment,
  Memory,
  Speed,
  Refresh,
  TrendingUp,
  TrendingDown,
  Schedule
} from '@mui/icons-material';
import { MetricsCardsProps } from '../../types';
import { format } from 'date-fns';

const MetricsCards: React.FC<MetricsCardsProps> = ({
  stats,
  isLoading = false
}) => {
  if (isLoading) {
    return (
      <Grid container spacing={3}>
        {[1, 2, 3, 4].map((index) => (
          <Grid item xs={12} sm={6} md={3} key={index}>
            <Card sx={{ height: 140 }}>
              <CardContent>
                <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
                  <Skeleton variant="circular" width={40} height={40} />
                  <Skeleton variant="rectangular" width={60} height={20} />
                </Box>
                <Skeleton variant="text" width="80%" height={32} sx={{ mt: 2 }} />
                <Skeleton variant="text" width="60%" height={20} sx={{ mt: 1 }} />
              </CardContent>
            </Card>
          </Grid>
        ))}
      </Grid>
    );
  }

  if (!stats) {
    return (
      <Grid container spacing={3}>
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Box
                sx={{
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  minHeight: 100,
                  color: 'text.secondary'
                }}
              >
                <Typography>No system statistics available</Typography>
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    );
  }

  // Calculate derived metrics
  const resolvedPercentage = stats.totalDriftEvents > 0 ? 
    Math.round((stats.resolvedDriftEvents / stats.totalDriftEvents) * 100) : 0;
  
  const activeIssues = stats.totalDriftEvents - stats.resolvedDriftEvents;
  
  // Format uptime
  const formatUptime = (uptimeHours: number) => {
    const days = Math.floor(uptimeHours / 24);
    const hours = Math.floor(uptimeHours % 24);
    if (days > 0) {
      return `${days}d ${hours}h`;
    }
    return `${hours}h`;
  };

  // Get status color based on value and thresholds
  const getStatusColor = (value: number, type: 'percentage' | 'usage' | 'health') => {
    switch (type) {
      case 'percentage':
        return value >= 80 ? 'success' : value >= 60 ? 'warning' : 'error';
      case 'usage':
        return value <= 70 ? 'success' : value <= 85 ? 'warning' : 'error';
      case 'health':
        return value >= 85 ? 'success' : value >= 70 ? 'warning' : 'error';
      default:
        return 'primary';
    }
  };

  // Metric card component
  const MetricCard: React.FC<{
    title: string;
    value: string | number;
    subtitle?: string;
    icon: React.ReactNode;
    trend?: 'up' | 'down' | 'stable';
    trendValue?: number;
    color?: 'primary' | 'success' | 'warning' | 'error';
    onClick?: () => void;
  }> = ({ title, value, subtitle, icon, trend, trendValue, color = 'primary', onClick }) => {
    const getTrendIcon = () => {
      switch (trend) {
        case 'up':
          return <TrendingUp sx={{ fontSize: 16, color: 'success.main' }} />;
        case 'down':
          return <TrendingDown sx={{ fontSize: 16, color: 'error.main' }} />;
        default:
          return null;
      }
    };

    return (
      <Card 
        sx={{ 
          height: 140, 
          cursor: onClick ? 'pointer' : 'default',
          transition: 'box-shadow 0.3s ease-in-out',
          '&:hover': onClick ? {
            boxShadow: 3
          } : {}
        }}
        onClick={onClick}
      >
        <CardContent>
          <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
            <Box
              sx={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                width: 40,
                height: 40,
                borderRadius: 1,
                bgcolor: `${color}.light`,
                color: `${color}.main`
              }}
            >
              {icon}
            </Box>
            {trend && (
              <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
                {getTrendIcon()}
                {trendValue && (
                  <Typography variant="caption" color="text.secondary">
                    {Math.abs(trendValue)}%
                  </Typography>
                )}
              </Box>
            )}
          </Box>
          
          <Typography variant="h4" sx={{ mt: 2, fontWeight: 'bold' }}>
            {value}
          </Typography>
          
          <Typography variant="body2" color="text.secondary" sx={{ mt: 0.5 }}>
            {title}
          </Typography>
          
          {subtitle && (
            <Typography variant="caption" color="text.secondary" sx={{ mt: 0.5, display: 'block' }}>
              {subtitle}
            </Typography>
          )}
        </CardContent>
      </Card>
    );
  };

  return (
    <>
      <Grid container spacing={3} sx={{ mb: 3 }}>
        {/* Total Repositories */}
        <Grid item xs={12} sm={6} md={3}>
          <MetricCard
            title="Total Repositories"
            value={stats.totalRepositories}
            subtitle="Active projects"
            icon={<Assessment />}
            color="primary"
          />
        </Grid>

        {/* Active Scans */}
        <Grid item xs={12} sm={6} md={3}>
          <MetricCard
            title="Active Scans"
            value={stats.activeScans}
            subtitle="Currently running"
            icon={<Refresh sx={{ animation: stats.activeScans > 0 ? 'spin 2s linear infinite' : 'none' }} />}
            color={stats.activeScans > 0 ? 'warning' : 'success'}
          />
        </Grid>

        {/* Total Drift Events */}
        <Grid item xs={12} sm={6} md={3}>
          <MetricCard
            title="Total Drift Events"
            value={stats.totalDriftEvents}
            subtitle={`${activeIssues} active, ${stats.resolvedDriftEvents} resolved`}
            icon={<BugReport />}
            color={activeIssues > 10 ? 'error' : activeIssues > 5 ? 'warning' : 'success'}
          />
        </Grid>

        {/* Resolution Rate */}
        <Grid item xs={12} sm={6} md={3}>
          <MetricCard
            title="Resolution Rate"
            value={`${resolvedPercentage}%`}
            subtitle={`${stats.resolvedDriftEvents} of ${stats.totalDriftEvents} resolved`}
            icon={<CheckCircle />}
            color={getStatusColor(resolvedPercentage, 'percentage')}
          />
        </Grid>
      </Grid>

      <Grid container spacing={3}>
        {/* System Health */}
        <Grid item xs={12} sm={6} md={3}>
          <MetricCard
            title="System Health"
            value={`${Math.round(stats.systemHealth)}%`}
            subtitle="Overall status"
            icon={<Assessment />}
            color={getStatusColor(stats.systemHealth, 'health')}
          />
        </Grid>

        {/* System Uptime */}
        <Grid item xs={12} sm={6} md={3}>
          <MetricCard
            title="System Uptime"
            value={formatUptime(stats.uptime)}
            subtitle="Continuous operation"
            icon={<Schedule />}
            color="success"
          />
        </Grid>

        {/* Memory Usage */}
        <Grid item xs={12} sm={6} md={3}>
          <MetricCard
            title="Memory Usage"
            value={`${Math.round(stats.memoryUsage)}%`}
            subtitle="RAM utilization"
            icon={<Memory />}
            color={getStatusColor(stats.memoryUsage, 'usage')}
          />
        </Grid>

        {/* CPU Usage */}
        <Grid item xs={12} sm={6} md={3}>
          <MetricCard
            title="CPU Usage"
            value={`${Math.round(stats.cpuUsage)}%`}
            subtitle="Processing load"
            icon={<Speed />}
            color={getStatusColor(stats.cpuUsage, 'usage')}
          />
        </Grid>
      </Grid>

      {/* Last Update Info */}
      <Box sx={{ mt: 3, display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <Typography variant="caption" color="text.secondary">
          Last updated: {format(new Date(stats.lastUpdate), 'MMM dd, yyyy HH:mm:ss')}
        </Typography>
        
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          {/* Status indicators */}
          <Tooltip title="System Status">
            <Chip
              size="small"
              icon={stats.systemHealth >= 85 ? <CheckCircle /> : <BugReport />}
              label={stats.systemHealth >= 85 ? 'Healthy' : 'Degraded'}
              color={stats.systemHealth >= 85 ? 'success' : 'error'}
              variant="outlined"
            />
          </Tooltip>
          
          <Tooltip title="Active Issues">
            <Chip
              size="small"
              label={`${activeIssues} issues`}
              color={activeIssues === 0 ? 'success' : activeIssues <= 5 ? 'warning' : 'error'}
              variant="outlined"
            />
          </Tooltip>
        </Box>
      </Box>

      {/* Custom styles for spin animation */}
      <style>
        {`
          @keyframes spin {
            from { transform: rotate(0deg); }
            to { transform: rotate(360deg); }
          }
        `}
      </style>
    </>
  );
};

export default MetricsCards;