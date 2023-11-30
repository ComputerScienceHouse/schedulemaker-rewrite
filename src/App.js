import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Index from './pages/index';
import Generate from './pages/generate';
import Browse from './pages/browse';
import Search from './pages/search';
import './App.css';

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Index />} />
        <Route path="/generate" element={<Generate />} />
        <Route path="/browse" element={<Browse />} />
        <Route path="/search" element={<Search />} />
      </Routes>
    </Router>
  );
}

export default App;
