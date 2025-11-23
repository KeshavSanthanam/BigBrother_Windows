import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';
import ThemeToggle from '../components/ThemeToggle';

export default function Settings() {
  const navigate = useNavigate();
  const [apiKey, setApiKey] = useState('');
  const [savedApiKey, setSavedApiKey] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    loadApiKey();
  }, []);

  const loadApiKey = async () => {
    try {
      const key = await invoke<string | null>('get_claude_api_key');
      setSavedApiKey(key);
      if (key) {
        setApiKey(key);
      }
    } catch (error) {
      console.error('Failed to load API key:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async (e: React.FormEvent) => {
    e.preventDefault();
    setSaving(true);

    try {
      await invoke('set_claude_api_key', { apiKey });
      setSavedApiKey(apiKey);
      alert('API key saved successfully!');
    } catch (error) {
      alert(`Failed to save API key: ${error}`);
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="flex justify-between items-center mb-8">
          <div>
            <button
              onClick={() => navigate('/')}
              className="text-primary dark:text-purple hover:underline mb-4"
            >
              ← Back to Dashboard
            </button>
            <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Settings</h1>
            <p className="text-gray-600 dark:text-gray-300 mt-1">Configure your BigBrother app</p>
          </div>
          <ThemeToggle />
        </div>

        <div className="space-y-6">
          {/* API Key Section */}
          <div className="card">
            <h2 className="text-xl font-semibold mb-4 dark:text-white">Claude API Key</h2>
            <p className="text-gray-600 dark:text-gray-400 text-sm mb-4">
              Your Claude API key is required for AI-powered task verification.
              Get your API key from{' '}
              <a
                href="https://console.anthropic.com/"
                target="_blank"
                rel="noopener noreferrer"
                className="text-primary dark:text-purple hover:underline"
              >
                Anthropic Console
              </a>
            </p>

            {loading ? (
              <div className="text-gray-500 dark:text-gray-400">Loading...</div>
            ) : (
              <form onSubmit={handleSave} className="space-y-4">
                <div>
                  <label className="label">API Key</label>
                  <input
                    type="password"
                    value={apiKey}
                    onChange={(e) => setApiKey(e.target.value)}
                    className="input font-mono"
                    placeholder="sk-ant-..."
                  />
                  {savedApiKey && (
                    <p className="text-xs text-green-600 dark:text-green-400 mt-1">
                      ✓ API key is configured
                    </p>
                  )}
                </div>

                <div className="flex gap-3">
                  <button
                    type="submit"
                    disabled={saving || !apiKey}
                    className="btn btn-primary bg-purple hover:bg-purple-dark disabled:opacity-50"
                  >
                    {saving ? 'Saving...' : 'Save API Key'}
                  </button>
                  {savedApiKey && apiKey !== savedApiKey && (
                    <button
                      type="button"
                      onClick={() => setApiKey(savedApiKey)}
                      className="btn btn-secondary"
                    >
                      Reset
                    </button>
                  )}
                </div>
              </form>
            )}

            <div className="mt-6 p-4 bg-yellow-50 dark:bg-yellow-900/20 rounded-lg">
              <p className="text-sm text-yellow-800 dark:text-yellow-200">
                <strong>Note:</strong> Your API key is stored securely in the local database.
                Never share your API key with others.
              </p>
            </div>
          </div>

          {/* Theme Section */}
          <div className="card">
            <h2 className="text-xl font-semibold mb-4 dark:text-white">Appearance</h2>
            <div className="flex items-center justify-between">
              <div>
                <p className="text-gray-900 dark:text-white font-medium">Dark Mode</p>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Toggle between light and dark themes
                </p>
              </div>
              <ThemeToggle />
            </div>
          </div>

          {/* Recording Settings */}
          <div className="card">
            <h2 className="text-xl font-semibold mb-4 dark:text-white">Recording Settings</h2>
            <div className="space-y-3 text-sm">
              <div className="flex justify-between">
                <span className="text-gray-600 dark:text-gray-400">Frame Rate:</span>
                <span className="dark:text-gray-300">5 fps (optimized for performance)</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600 dark:text-gray-400">Video Codec:</span>
                <span className="dark:text-gray-300">H.264 (libx264)</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600 dark:text-gray-400">Quality Preset:</span>
                <span className="dark:text-gray-300">Very Fast</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600 dark:text-gray-400">Max Duration:</span>
                <span className="dark:text-gray-300">4 hours (240 minutes)</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
