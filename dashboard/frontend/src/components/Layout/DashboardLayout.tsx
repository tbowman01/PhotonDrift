import React, { useState } from 'react';
import {
  Box,
  AppBar,
  Toolbar,
  Typography,
  IconButton,
  Drawer,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Divider,
  Badge,
  Menu,
  MenuItem,
  Avatar,
  Switch,
  FormControlLabel,
  Chip,
  Tooltip,
  useTheme,
  useMediaQuery
} from '@mui/material';
import {
  Menu as MenuIcon,
  Dashboard,
  Assessment,
  BugReport,
  Settings,
  AccountCircle,
  Notifications,
  Brightness4,
  Brightness7,
  Refresh,
  WifiOff,
  Wifi,
  Close
} from '@mui/icons-material';
import { useStore } from '../../stores/useStore';
import { useWebSocket } from '../../services/websocketService';

interface DashboardLayoutProps {
  children: React.ReactNode;
}

const DRAWER_WIDTH = 240;

const DashboardLayout: React.FC<DashboardLayoutProps> = ({ children }) => {
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down('md'));
  
  const [mobileOpen, setMobileOpen] = useState(false);
  const [profileAnchorEl, setProfileAnchorEl] = useState<null | HTMLElement>(null);
  const [notificationAnchorEl, setNotificationAnchorEl] = useState<null | HTMLElement>(null);

  // Store state
  const {
    user,
    selectedRepository,
    theme: currentTheme,
    setTheme,
    alerts,
    removeAlert
  } = useStore();

  const { isConnected, connect, disconnect } = useWebSocket();

  // Navigation items
  const navigationItems = [
    { text: 'Dashboard', icon: <Dashboard />, id: 'dashboard', active: true },
    { text: 'Analytics', icon: <Assessment />, id: 'analytics', active: false },
    { text: 'Drift Events', icon: <BugReport />, id: 'events', active: false },
    { text: 'Settings', icon: <Settings />, id: 'settings', active: false },
  ];

  // Get unread notifications count
  const unreadAlertsCount = alerts.filter(alert => !alert.acknowledged).length;

  // Handle drawer toggle
  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
  };

  // Handle profile menu
  const handleProfileClick = (event: React.MouseEvent<HTMLElement>) => {
    setProfileAnchorEl(event.currentTarget);
  };

  const handleProfileClose = () => {
    setProfileAnchorEl(null);
  };

  // Handle notifications
  const handleNotificationsClick = (event: React.MouseEvent<HTMLElement>) => {
    setNotificationAnchorEl(event.currentTarget);
  };

  const handleNotificationsClose = () => {
    setNotificationAnchorEl(null);
  };

  const handleDismissAlert = (alertId: string) => {
    removeAlert(alertId);
  };

  // Handle theme toggle
  const handleThemeToggle = () => {
    setTheme(currentTheme === 'light' ? 'dark' : 'light');
  };

  // Handle connection toggle
  const handleConnectionToggle = () => {
    if (isConnected) {
      disconnect();
    } else {
      connect();
    }
  };

  // Navigation drawer content
  const drawerContent = (
    <Box sx={{ width: DRAWER_WIDTH }}>
      {/* Logo/Brand */}
      <Box sx={{ p: 2, display: 'flex', alignItems: 'center', gap: 2 }}>
        <Box
          sx={{
            width: 32,
            height: 32,
            borderRadius: 1,
            bgcolor: 'primary.main',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            color: 'white'
          }}
        >
          <Assessment />
        </Box>
        <Typography variant="h6" fontWeight="bold">
          PhotonDrift
        </Typography>
      </Box>

      <Divider />

      {/* Repository Info */}
      {selectedRepository && (
        <Box sx={{ p: 2 }}>
          <Typography variant="caption" color="text.secondary" sx={{ textTransform: 'uppercase', letterSpacing: 1 }}>
            Current Repository
          </Typography>
          <Box sx={{ mt: 1, display: 'flex', alignItems: 'center', gap: 1 }}>
            <Typography variant="body2" fontWeight="medium">
              {selectedRepository.name}
            </Typography>
            <Chip
              size="small"
              label={selectedRepository.isActive ? 'Active' : 'Inactive'}
              color={selectedRepository.isActive ? 'success' : 'default'}
            />
          </Box>
        </Box>
      )}

      <Divider />

      {/* Navigation */}
      <List sx={{ px: 1 }}>
        {navigationItems.map((item) => (
          <ListItem key={item.id} disablePadding>
            <ListItemButton
              selected={item.active}
              sx={{
                borderRadius: 1,
                mx: 1,
                '&.Mui-selected': {
                  bgcolor: 'primary.light',
                  color: 'primary.contrastText',
                  '&:hover': {
                    bgcolor: 'primary.light',
                  }
                }
              }}
            >
              <ListItemIcon sx={{ color: 'inherit', minWidth: 36 }}>
                {item.icon}
              </ListItemIcon>
              <ListItemText primary={item.text} />
            </ListItemButton>
          </ListItem>
        ))}
      </List>

      <Box sx={{ flexGrow: 1 }} />

      {/* Connection Status */}
      <Box sx={{ p: 2 }}>
        <FormControlLabel
          control={
            <Switch
              checked={isConnected}
              onChange={handleConnectionToggle}
              size="small"
            />
          }
          label={
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
              {isConnected ? <Wifi fontSize="small" /> : <WifiOff fontSize="small" />}
              <Typography variant="caption">
                {isConnected ? 'Connected' : 'Disconnected'}
              </Typography>
            </Box>
          }
        />
      </Box>
    </Box>
  );

  return (
    <Box sx={{ display: 'flex' }}>
      {/* App Bar */}
      <AppBar
        position="fixed"
        sx={{
          width: { md: `calc(100% - ${DRAWER_WIDTH}px)` },
          ml: { md: `${DRAWER_WIDTH}px` },
          zIndex: theme.zIndex.drawer + 1
        }}
      >
        <Toolbar>
          <IconButton
            color="inherit"
            edge="start"
            onClick={handleDrawerToggle}
            sx={{ mr: 2, display: { md: 'none' } }}
          >
            <MenuIcon />
          </IconButton>

          <Typography variant="h6" noWrap component="div" sx={{ flexGrow: 1 }}>
            Visual Analytics Dashboard
          </Typography>

          {/* Connection Status Indicator */}
          <Tooltip title={isConnected ? 'Connected to server' : 'Disconnected from server'}>
            <IconButton color="inherit">
              {isConnected ? (
                <Wifi sx={{ color: 'success.light' }} />
              ) : (
                <WifiOff sx={{ color: 'error.light' }} />
              )}
            </IconButton>
          </Tooltip>

          {/* Refresh Button */}
          <Tooltip title="Refresh Data">
            <IconButton color="inherit">
              <Refresh />
            </IconButton>
          </Tooltip>

          {/* Theme Toggle */}
          <Tooltip title="Toggle Theme">
            <IconButton color="inherit" onClick={handleThemeToggle}>
              {currentTheme === 'light' ? <Brightness4 /> : <Brightness7 />}
            </IconButton>
          </Tooltip>

          {/* Notifications */}
          <IconButton color="inherit" onClick={handleNotificationsClick}>
            <Badge badgeContent={unreadAlertsCount} color="error">
              <Notifications />
            </Badge>
          </IconButton>

          {/* Profile */}
          <IconButton color="inherit" onClick={handleProfileClick}>
            <Avatar sx={{ width: 32, height: 32, bgcolor: 'primary.dark' }}>
              {user ? user.name.charAt(0).toUpperCase() : <AccountCircle />}
            </Avatar>
          </IconButton>
        </Toolbar>
      </AppBar>

      {/* Navigation Drawer */}
      <Box component="nav" sx={{ width: { md: DRAWER_WIDTH }, flexShrink: { md: 0 } }}>
        {/* Mobile drawer */}
        <Drawer
          variant="temporary"
          open={mobileOpen}
          onClose={handleDrawerToggle}
          ModalProps={{ keepMounted: true }}
          sx={{
            display: { xs: 'block', md: 'none' },
            '& .MuiDrawer-paper': { boxSizing: 'border-box', width: DRAWER_WIDTH },
          }}
        >
          {drawerContent}
        </Drawer>

        {/* Desktop drawer */}
        <Drawer
          variant="permanent"
          sx={{
            display: { xs: 'none', md: 'block' },
            '& .MuiDrawer-paper': { boxSizing: 'border-box', width: DRAWER_WIDTH },
          }}
          open
        >
          {drawerContent}
        </Drawer>
      </Box>

      {/* Profile Menu */}
      <Menu
        anchorEl={profileAnchorEl}
        open={Boolean(profileAnchorEl)}
        onClose={handleProfileClose}
        transformOrigin={{ horizontal: 'right', vertical: 'top' }}
        anchorOrigin={{ horizontal: 'right', vertical: 'bottom' }}
      >
        <MenuItem>
          <Avatar sx={{ width: 24, height: 24, mr: 2 }} />
          {user ? user.name : 'Guest User'}
        </MenuItem>
        <Divider />
        <MenuItem onClick={handleProfileClose}>Profile Settings</MenuItem>
        <MenuItem onClick={handleProfileClose}>Account</MenuItem>
        <MenuItem onClick={handleProfileClose}>Sign Out</MenuItem>
      </Menu>

      {/* Notifications Menu */}
      <Menu
        anchorEl={notificationAnchorEl}
        open={Boolean(notificationAnchorEl)}
        onClose={handleNotificationsClose}
        transformOrigin={{ horizontal: 'right', vertical: 'top' }}
        anchorOrigin={{ horizontal: 'right', vertical: 'bottom' }}
        PaperProps={{
          sx: { width: 320, maxHeight: 400 }
        }}
      >
        <Box sx={{ p: 2, borderBottom: '1px solid', borderColor: 'divider' }}>
          <Typography variant="h6">
            Notifications ({unreadAlertsCount})
          </Typography>
        </Box>
        {alerts.length === 0 ? (
          <MenuItem>
            <Typography variant="body2" color="text.secondary">
              No notifications
            </Typography>
          </MenuItem>
        ) : (
          alerts.slice(0, 5).map((alert) => (
            <MenuItem key={alert.id} sx={{ flexDirection: 'column', alignItems: 'flex-start' }}>
              <Box sx={{ display: 'flex', justifyContent: 'space-between', width: '100%', alignItems: 'flex-start' }}>
                <Box sx={{ flex: 1 }}>
                  <Typography variant="body2" fontWeight="medium">
                    {alert.title}
                  </Typography>
                  <Typography variant="caption" color="text.secondary">
                    {alert.message}
                  </Typography>
                  <Typography variant="caption" color="text.secondary" sx={{ display: 'block', mt: 0.5 }}>
                    {new Date(alert.timestamp).toLocaleTimeString()}
                  </Typography>
                </Box>
                <IconButton
                  size="small"
                  onClick={(e) => {
                    e.stopPropagation();
                    handleDismissAlert(alert.id);
                  }}
                >
                  <Close fontSize="small" />
                </IconButton>
              </Box>
            </MenuItem>
          ))
        )}
        {alerts.length > 5 && (
          <MenuItem onClick={handleNotificationsClose}>
            <Typography variant="body2" color="primary.main">
              View all notifications
            </Typography>
          </MenuItem>
        )}
      </Menu>

      {/* Main Content */}
      <Box
        component="main"
        sx={{
          flexGrow: 1,
          width: { md: `calc(100% - ${DRAWER_WIDTH}px)` },
          mt: '64px', // AppBar height
          p: 3,
          minHeight: 'calc(100vh - 64px)',
          bgcolor: 'background.default'
        }}
      >
        {children}
      </Box>
    </Box>
  );
};

export default DashboardLayout;