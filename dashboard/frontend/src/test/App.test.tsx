import { describe, it, expect } from 'vitest';
import { render as rtlRender, screen } from '@testing-library/react';
import App from '../App';

describe('App Component', () => {
  it('renders without crashing', () => {
    rtlRender(<App />);
    expect(screen.getByText('Vite + React')).toBeInTheDocument();
  });

  it('displays the initial count', () => {
    rtlRender(<App />);
    expect(screen.getByText('count is 0')).toBeInTheDocument();
  });

  it('displays Vite and React logos', () => {
    rtlRender(<App />);
    
    const viteLogo = screen.getByAltText('Vite logo');
    const reactLogo = screen.getByAltText('React logo');
    
    expect(viteLogo).toBeInTheDocument();
    expect(reactLogo).toBeInTheDocument();
  });

  it('contains links to documentation', () => {
    rtlRender(<App />);
    
    const links = screen.getAllByRole('link');
    expect(links.length).toBeGreaterThan(0);
  });
});