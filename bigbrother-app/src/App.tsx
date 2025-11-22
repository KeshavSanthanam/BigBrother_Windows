import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Dashboard from './pages/Dashboard';
import RecordingPage from './pages/RecordingPage';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/recording/:taskId" element={<RecordingPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
