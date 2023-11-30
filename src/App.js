import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Index from './pages/index';
import Generate from './pages/generate';
import './App.css';

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Index />} />
        <Route path="/generate" element={<Generate />} />
      </Routes>
    </Router>
  );
}

export default App;
