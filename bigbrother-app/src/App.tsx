import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Dashboard from './pages/Dashboard';
import RecordingPage from './pages/RecordingPage';
import TaskHistory from './pages/TaskHistory';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/recording/:taskId" element={<RecordingPage />} />
        <Route path="/history" element={<TaskHistory />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
