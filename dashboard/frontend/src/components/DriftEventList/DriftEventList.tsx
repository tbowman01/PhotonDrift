import React, { useState, useMemo } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  TablePagination,
  TableSortLabel,
  TextField,
  InputAdornment,
  Chip,
  IconButton,
  Button,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  SelectChangeEvent,
  OutlinedInput,
  Checkbox,
  ListItemText,
  Tooltip,
  Skeleton,
  Collapse
} from '@mui/material';
import {
  Search,
  FilterList,
  CheckCircle,
  RadioButtonUnchecked,
  ExpandMore,
  ExpandLess,
  OpenInNew,
  BugReport,
  Code,
  Assignment
} from '@mui/icons-material';
import { DriftEventListProps, DriftEvent, SortConfig } from '../../types';
import { format } from 'date-fns';

const DriftEventList: React.FC<DriftEventListProps> = ({
  events,
  onEventClick,
  onResolve,
  filters,
  onFiltersChange,
  isLoading = false
}) => {
  const [page, setPage] = useState(0);
  const [rowsPerPage, setRowsPerPage] = useState(25);
  const [sortConfig, setSortConfig] = useState<SortConfig>({ field: 'timestamp', direction: 'desc' });
  const [showFilters, setShowFilters] = useState(false);
  const [expandedEvent, setExpandedEvent] = useState<string | null>(null);

  // Severity options
  const severityOptions = ['low', 'medium', 'high', 'critical'];

  // Category options (derived from events)
  const categoryOptions = useMemo(() => {
    const categories = new Set(events.map(event => event.category));
    return Array.from(categories).sort();
  }, [events]);

  // Filtered and sorted events
  const processedEvents = useMemo(() => {
    let filtered = [...events];

    // Apply filters
    if (filters.severity.length > 0) {
      filtered = filtered.filter(event => filters.severity.includes(event.severity));
    }

    if (filters.category.length > 0) {
      filtered = filtered.filter(event => filters.category.includes(event.category));
    }

    if (filters.resolved !== null) {
      filtered = filtered.filter(event => event.resolved === filters.resolved);
    }

    if (filters.search) {
      const search = filters.search.toLowerCase();
      filtered = filtered.filter(event =>
        event.title.toLowerCase().includes(search) ||
        event.description.toLowerCase().includes(search) ||
        event.category.toLowerCase().includes(search) ||
        event.location.file.toLowerCase().includes(search)
      );
    }

    // Apply sorting
    filtered.sort((a, b) => {
      const aValue = a[sortConfig.field];
      const bValue = b[sortConfig.field];

      if (sortConfig.field === 'timestamp') {
        const aTime = new Date(aValue as Date).getTime();
        const bTime = new Date(bValue as Date).getTime();
        return sortConfig.direction === 'asc' ? aTime - bTime : bTime - aTime;
      }

      if (typeof aValue === 'string' && typeof bValue === 'string') {
        return sortConfig.direction === 'asc' 
          ? aValue.localeCompare(bValue)
          : bValue.localeCompare(aValue);
      }

      if (typeof aValue === 'number' && typeof bValue === 'number') {
        return sortConfig.direction === 'asc' ? aValue - bValue : bValue - aValue;
      }

      return 0;
    });

    return filtered;
  }, [events, filters, sortConfig]);

  // Handle sorting
  const handleSort = (field: keyof DriftEvent) => {
    setSortConfig(prev => ({
      field,
      direction: prev.field === field && prev.direction === 'asc' ? 'desc' : 'asc'
    }));
  };

  // Handle filter changes
  const handleSeverityChange = (event: SelectChangeEvent<string[]>) => {
    const value = event.target.value as string[];
    onFiltersChange({ ...filters, severity: value });
  };

  const handleCategoryChange = (event: SelectChangeEvent<string[]>) => {
    const value = event.target.value as string[];
    onFiltersChange({ ...filters, category: value });
  };

  const handleResolvedChange = (event: SelectChangeEvent<string>) => {
    const value = event.target.value;
    onFiltersChange({ 
      ...filters, 
      resolved: value === 'all' ? null : value === 'resolved' 
    });
  };

  const handleSearchChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    onFiltersChange({ ...filters, search: event.target.value });
  };

  // Handle pagination
  const handleChangePage = (event: unknown, newPage: number) => {
    setPage(newPage);
  };

  const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
    setRowsPerPage(parseInt(event.target.value, 10));
    setPage(0);
  };

  // Get severity color
  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'critical': return 'error';
      case 'high': return 'warning';
      case 'medium': return 'info';
      case 'low': return 'success';
      default: return 'default';
    }
  };

  // Loading state
  if (isLoading) {
    return (
      <Card>
        <CardContent>
          <Typography variant="h6" gutterBottom>
            Drift Events
          </Typography>
          <TableContainer>
            <Table>
              <TableHead>
                <TableRow>
                  {['Severity', 'Title', 'Category', 'Location', 'Time', 'Status'].map((header) => (
                    <TableCell key={header}>
                      <Skeleton variant="text" />
                    </TableCell>
                  ))}
                </TableRow>
              </TableHead>
              <TableBody>
                {[...Array(5)].map((_, index) => (
                  <TableRow key={index}>
                    {[...Array(6)].map((_, cellIndex) => (
                      <TableCell key={cellIndex}>
                        <Skeleton variant="text" />
                      </TableCell>
                    ))}
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>
        </CardContent>
      </Card>
    );
  }

  const paginatedEvents = processedEvents.slice(
    page * rowsPerPage,
    page * rowsPerPage + rowsPerPage
  );

  return (
    <Card>
      <CardContent>
        {/* Header */}
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 2 }}>
          <Typography variant="h6">
            Drift Events ({processedEvents.length})
          </Typography>
          <Button
            startIcon={<FilterList />}
            onClick={() => setShowFilters(!showFilters)}
            variant={showFilters ? 'contained' : 'outlined'}
            size="small"
          >
            Filters
          </Button>
        </Box>

        {/* Filters */}
        <Collapse in={showFilters}>
          <Box sx={{ mb: 3, p: 2, bgcolor: 'grey.50', borderRadius: 1 }}>
            <Typography variant="subtitle2" gutterBottom>
              FILTERS
            </Typography>
            <Box sx={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))', gap: 2 }}>
              {/* Search */}
              <TextField
                fullWidth
                size="small"
                placeholder="Search events..."
                value={filters.search}
                onChange={handleSearchChange}
                InputProps={{
                  startAdornment: (
                    <InputAdornment position="start">
                      <Search />
                    </InputAdornment>
                  )
                }}
              />

              {/* Severity Filter */}
              <FormControl size="small" fullWidth>
                <InputLabel>Severity</InputLabel>
                <Select
                  multiple
                  value={filters.severity}
                  onChange={handleSeverityChange}
                  input={<OutlinedInput label="Severity" />}
                  renderValue={(selected) => (
                    <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 0.5 }}>
                      {selected.map((value) => (
                        <Chip
                          key={value}
                          label={value}
                          size="small"
                          color={getSeverityColor(value) as any}
                        />
                      ))}
                    </Box>
                  )}
                >
                  {severityOptions.map((severity) => (
                    <MenuItem key={severity} value={severity}>
                      <Checkbox checked={filters.severity.indexOf(severity) > -1} />
                      <ListItemText primary={severity.charAt(0).toUpperCase() + severity.slice(1)} />
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>

              {/* Category Filter */}
              <FormControl size="small" fullWidth>
                <InputLabel>Category</InputLabel>
                <Select
                  multiple
                  value={filters.category}
                  onChange={handleCategoryChange}
                  input={<OutlinedInput label="Category" />}
                  renderValue={(selected) => (
                    <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 0.5 }}>
                      {selected.map((value) => (
                        <Chip key={value} label={value} size="small" />
                      ))}
                    </Box>
                  )}
                >
                  {categoryOptions.map((category) => (
                    <MenuItem key={category} value={category}>
                      <Checkbox checked={filters.category.indexOf(category) > -1} />
                      <ListItemText primary={category} />
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>

              {/* Status Filter */}
              <FormControl size="small" fullWidth>
                <InputLabel>Status</InputLabel>
                <Select
                  value={filters.resolved === null ? 'all' : filters.resolved ? 'resolved' : 'unresolved'}
                  onChange={handleResolvedChange}
                  label="Status"
                >
                  <MenuItem value="all">All</MenuItem>
                  <MenuItem value="resolved">Resolved</MenuItem>
                  <MenuItem value="unresolved">Unresolved</MenuItem>
                </Select>
              </FormControl>
            </Box>
          </Box>
        </Collapse>

        {/* Table */}
        <TableContainer>
          <Table size="small">
            <TableHead>
              <TableRow>
                <TableCell>
                  <TableSortLabel
                    active={sortConfig.field === 'severity'}
                    direction={sortConfig.direction}
                    onClick={() => handleSort('severity')}
                  >
                    Severity
                  </TableSortLabel>
                </TableCell>
                <TableCell>
                  <TableSortLabel
                    active={sortConfig.field === 'title'}
                    direction={sortConfig.direction}
                    onClick={() => handleSort('title')}
                  >
                    Title
                  </TableSortLabel>
                </TableCell>
                <TableCell>
                  <TableSortLabel
                    active={sortConfig.field === 'category'}
                    direction={sortConfig.direction}
                    onClick={() => handleSort('category')}
                  >
                    Category
                  </TableSortLabel>
                </TableCell>
                <TableCell>Location</TableCell>
                <TableCell>
                  <TableSortLabel
                    active={sortConfig.field === 'timestamp'}
                    direction={sortConfig.direction}
                    onClick={() => handleSort('timestamp')}
                  >
                    Time
                  </TableSortLabel>
                </TableCell>
                <TableCell>Status</TableCell>
                <TableCell align="right">Actions</TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {paginatedEvents.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={7} align="center" sx={{ py: 3 }}>
                    <Typography color="text.secondary">
                      No drift events found matching your filters
                    </Typography>
                  </TableCell>
                </TableRow>
              ) : (
                paginatedEvents.map((event) => (
                  <React.Fragment key={event.id}>
                    <TableRow
                      hover
                      sx={{ cursor: onEventClick ? 'pointer' : 'default' }}
                      onClick={onEventClick ? () => onEventClick(event) : undefined}
                    >
                      <TableCell>
                        <Chip
                          label={event.severity}
                          size="small"
                          color={getSeverityColor(event.severity) as any}
                          icon={event.severity === 'critical' ? <BugReport /> : undefined}
                        />
                      </TableCell>
                      <TableCell>
                        <Typography variant="body2" fontWeight="medium">
                          {event.title}
                        </Typography>
                        {event.mlScore && (
                          <Typography variant="caption" color="text.secondary">
                            ML Score: {Math.round(event.mlScore * 100)}%
                          </Typography>
                        )}
                      </TableCell>
                      <TableCell>
                        <Chip
                          label={event.category}
                          size="small"
                          variant="outlined"
                          icon={<Code />}
                        />
                      </TableCell>
                      <TableCell>
                        <Typography variant="body2">
                          {event.location.file}
                        </Typography>
                        {event.location.line && (
                          <Typography variant="caption" color="text.secondary">
                            Line {event.location.line}
                          </Typography>
                        )}
                      </TableCell>
                      <TableCell>
                        <Typography variant="body2">
                          {format(new Date(event.timestamp), 'MMM dd, HH:mm')}
                        </Typography>
                      </TableCell>
                      <TableCell>
                        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                          {event.resolved ? (
                            <CheckCircle sx={{ color: 'success.main', fontSize: 16 }} />
                          ) : (
                            <RadioButtonUnchecked sx={{ color: 'text.secondary', fontSize: 16 }} />
                          )}
                          <Typography variant="body2">
                            {event.resolved ? 'Resolved' : 'Open'}
                          </Typography>
                        </Box>
                      </TableCell>
                      <TableCell align="right">
                        <Box sx={{ display: 'flex', gap: 0.5 }}>
                          <Tooltip title="View Details">
                            <IconButton
                              size="small"
                              onClick={(e) => {
                                e.stopPropagation();
                                setExpandedEvent(expandedEvent === event.id ? null : event.id);
                              }}
                            >
                              {expandedEvent === event.id ? <ExpandLess /> : <ExpandMore />}
                            </IconButton>
                          </Tooltip>
                          {onResolve && !event.resolved && (
                            <Tooltip title="Mark as Resolved">
                              <IconButton
                                size="small"
                                onClick={(e) => {
                                  e.stopPropagation();
                                  onResolve(event.id);
                                }}
                              >
                                <CheckCircle />
                              </IconButton>
                            </Tooltip>
                          )}
                          <Tooltip title="Open in Editor">
                            <IconButton size="small">
                              <OpenInNew />
                            </IconButton>
                          </Tooltip>
                        </Box>
                      </TableCell>
                    </TableRow>
                    
                    {/* Expanded Details */}
                    <TableRow>
                      <TableCell colSpan={7} sx={{ py: 0, border: 0 }}>
                        <Collapse in={expandedEvent === event.id} timeout="auto">
                          <Box sx={{ p: 2, bgcolor: 'grey.50', mx: -1, my: 1, borderRadius: 1 }}>
                            <Typography variant="subtitle2" gutterBottom>
                              Description
                            </Typography>
                            <Typography variant="body2" sx={{ mb: 2 }}>
                              {event.description}
                            </Typography>
                            
                            {event.suggestion && (
                              <>
                                <Typography variant="subtitle2" gutterBottom>
                                  Suggested Fix
                                </Typography>
                                <Typography variant="body2" sx={{ mb: 2, color: 'primary.main' }}>
                                  {event.suggestion}
                                </Typography>
                              </>
                            )}
                            
                            <Box sx={{ display: 'flex', gap: 2, flexWrap: 'wrap' }}>
                              {event.tags.length > 0 && (
                                <Box>
                                  <Typography variant="caption" color="text.secondary">
                                    Tags:
                                  </Typography>
                                  <Box sx={{ display: 'flex', gap: 0.5, mt: 0.5 }}>
                                    {event.tags.map((tag, index) => (
                                      <Chip key={index} label={tag} size="small" variant="outlined" />
                                    ))}
                                  </Box>
                                </Box>
                              )}
                              
                              {event.assignee && (
                                <Box>
                                  <Typography variant="caption" color="text.secondary">
                                    Assignee:
                                  </Typography>
                                  <Typography variant="body2" sx={{ mt: 0.5 }}>
                                    {event.assignee}
                                  </Typography>
                                </Box>
                              )}
                              
                              {event.confidence && (
                                <Box>
                                  <Typography variant="caption" color="text.secondary">
                                    Confidence:
                                  </Typography>
                                  <Typography variant="body2" sx={{ mt: 0.5 }}>
                                    {Math.round(event.confidence * 100)}%
                                  </Typography>
                                </Box>
                              )}
                            </Box>
                          </Box>
                        </Collapse>
                      </TableCell>
                    </TableRow>
                  </React.Fragment>
                ))
              )}
            </TableBody>
          </Table>
        </TableContainer>

        {/* Pagination */}
        <TablePagination
          component="div"
          count={processedEvents.length}
          page={page}
          onPageChange={handleChangePage}
          rowsPerPage={rowsPerPage}
          onRowsPerPageChange={handleChangeRowsPerPage}
          rowsPerPageOptions={[10, 25, 50, 100]}
          showFirstButton
          showLastButton
        />
      </CardContent>
    </Card>
  );
};

export default DriftEventList;