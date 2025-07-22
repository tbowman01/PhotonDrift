import React, { useEffect, useMemo } from 'react';
import { ThemeProvider, createTheme, CssBaseline, Box, Grid, Paper } from '@mui/material';
import { DashboardLayout } from './components/Layout';
import DriftTimeline from './components/DriftTimeline';
import ArchitectureHealth from './components/ArchitectureHealth';
import MetricsCards from './components/MetricsCards';
import DriftEventList from './components/DriftEventList';
import { useStore, useFilteredDriftEvents } from './stores/useStore';
import { useWebSocket } from './services/websocketService';
import { websocketService } from './services/websocketService';
import './App.css';

function App() {
  const {
    theme: themeMode,
    user,
    selectedRepository,
    architectureHealth,
    systemStats,
    filters,
    selectedTimeRange,
    setFilters,
    setUser,
    setSelectedRepository
  } = useStore();

  const { connect } = useWebSocket();
  const filteredDriftEvents = useFilteredDriftEvents();

  // Create Material-UI theme
  const theme = useMemo(
    () =>
      createTheme({
        palette: {
          mode: themeMode,
          primary: {
            main: '#1976d2',
          },
          secondary: {
            main: '#dc004e',
          },
          background: {
            default: themeMode === 'light' ? '#f5f5f5' : '#121212',
            paper: themeMode === 'light' ? '#ffffff' : '#1e1e1e',
          },
        },
        typography: {
          fontFamily: '"Roboto", "Helvetica", "Arial", sans-serif',
          h6: {
            fontWeight: 600,
          },
        },
        components: {
          MuiCard: {
            styleOverrides: {
              root: {
                borderRadius: 12,
                boxShadow: '0 2px 10px rgba(0,0,0,0.1)',
              },
            },
          },
          MuiButton: {
            styleOverrides: {
              root: {
                borderRadius: 8,
                textTransform: 'none',
              },
            },
          },
        },
      }),
    [themeMode]
  );

  // Initialize the app
  useEffect(() => {
    // Set mock user data
    setUser({
      id: '1',
      email: 'user@photondrift.dev',
      name: 'Demo User',
      role: 'admin',
      preferences: {
        theme: 'light',
        notifications: true,
        defaultView: 'dashboard',
        refreshInterval: 30,
        chartColors: ['#1976d2', '#dc004e', '#388e3c', '#f57c00']
      }
    });

    // Set mock repository
    setSelectedRepository({
      id: 'repo-1',
      name: 'photon-drift-demo',
      path: '/projects/photon-drift-demo',
      description: 'Demo repository for PhotonDrift analytics',
      isActive: true,
      configuration: {
        adrDirectory: 'docs/adr',
        enableAutoScan: true,
        scanInterval: 60,
        mlThreshold: 0.7,
        excludePatterns: ['*.test.*', 'node_modules/**'],
        includePatterns: ['**/*.ts', '**/*.js', '**/*.tsx', '**/*.jsx'],
        notifications: {
          enabled: true,
          emailAlerts: true,
          criticalThreshold: 0.9,
          highThreshold: 0.7
        }
      }
    });

    // Connect to WebSocket
    connect();

    return () => {
      websocketService.disconnect();
    };
  }, [setUser, setSelectedRepository, connect]);

  // Handle event click
  const handleEventClick = (event: any) => {
    console.log('Event clicked:', event);
    // In a real app, this might open a modal or navigate to a detail page
  };

  // Handle resolve event
  const handleResolveEvent = (eventId: string) => {
    console.log('Resolving event:', eventId);
    // In a real app, this would make an API call to resolve the event
  };

  // Handle filter changes
  const handleFiltersChange = (newFilters: any) => {
    setFilters(newFilters);
  };

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <DashboardLayout>
        <Box sx={{ flexGrow: 1 }}>
          {/* Metrics Cards */}
          <Box sx={{ mb: 3 }}>
            {systemStats && (
              <MetricsCards stats={systemStats} isLoading={false} />
            )}
          </Box>

          {/* Main Dashboard Content */}
          <Grid container spacing={3}>
            {/* Architecture Health - Left Column */}
            <Grid item xs={12} lg={4}>
              <Paper sx={{ height: 'fit-content' }}>
                {architectureHealth ? (
                  <ArchitectureHealth 
                    health={architectureHealth} 
                    isLoading={false}
                    showTrends={true}
                  />
                ) : (
                  <ArchitectureHealth 
                    health={{} as any} 
                    isLoading={true}
                    showTrends={true}
                  />
                )}
              </Paper>
            </Grid>

            {/* Timeline Chart - Right Column */}
            <Grid item xs={12} lg={8}>
              <Paper sx={{ p: 2, height: '500px' }}>
                <DriftTimeline
                  events={filteredDriftEvents}
                  height={460}
                  onEventClick={handleEventClick}
                  selectedTimeRange={selectedTimeRange}
                  isLoading={false}
                />
              </Paper>
            </Grid>
          </Grid>

          {/* Drift Events Table */}
          <Box sx={{ mt: 3 }}>
            <DriftEventList
              events={filteredDriftEvents}
              onEventClick={handleEventClick}
              onResolve={handleResolveEvent}
              filters={filters}
              onFiltersChange={handleFiltersChange}
              isLoading={false}
            />
          </Box>
        </Box>
      </DashboardLayout>
    </ThemeProvider>
  );
}

export default App;