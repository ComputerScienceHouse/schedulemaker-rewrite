import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import Index from "./pages/index";
import Generate from "./pages/generate";
import Browse from "./pages/browse";
import Search from "./pages/search";
import Header from './components/header';
import Footer from './components/footer';

function App() {
  return (
    <Router>
      <div id="superContainer">
        <div>&nbsp;</div>
        <div>&nbsp;</div>
        <Header />
        <Routes>
          <Route path="/" element={<Index />} />
          <Route path="/generate" element={<Generate />} />
          <Route path="/browse" element={<Browse />} />
          <Route path="/search" element={<Search />} />
        </Routes>
        <Footer />
      </div>
    </Router>
  );
}

export default App;
