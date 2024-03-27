import React from 'react';
import Navbar from './components/Navbar';
import Home from './components/Home';
import About from './pages/About';
import Profile from './pages/Profile';
import './App.css';

function App() {
  return (
    <div className="App">
      <Navbar />
      <div className="content">
        <Home />
        <About />
        <Profile />
      </div>
    </div>
  );
}

export default App;
