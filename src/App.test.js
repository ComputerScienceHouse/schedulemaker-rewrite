import { render, screen } from '@testing-library/react';
import App from './App';

test('renders version in footer', () => {
  render(<App />);
  const linkElement = screen.getByText(/Version: 4.0.0/i);
  expect(linkElement).toBeInTheDocument();
});
