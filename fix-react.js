const fs = require('fs');
const content = `import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import { Github, Plus, Trash2, CheckCircle2, Copy, ExternalLink, Loader2, RefreshCw } from 'lucide-react';
import { formatDistanceToNow } from 'date-fns';

export default function App() {
  const [providers, setProviders] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  
  // Auth flow state
  const [isAuthenticating, setIsAuthenticating] = useState(false);
  const [deviceCode, setDeviceCode] = useState(null);
  const [copied, setCopied] = useState(false);
  
  const CLIENT_ID = "Ov23liC8F2cQhuOFCxLu";

  const fetchProviders = async () => {
    try {
      setLoading(true);
      const data = await invoke('list_providers');
      setProviders(data);
      setError(null);
    } catch (e) {
      setError(e.toString());
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchProviders();
  }, []);

  const handleStartAuth = async () => {
    try {
      setIsAuthenticating(true);
      setError(null);
      
      const response = await invoke('start_device_flow', { clientId: CLIENT_ID });
      setDeviceCode(response);
      
      // Start polling
      const provider = await invoke('complete_device_flow', {
        clientId: CLIENT_ID,
        deviceCode: response.device_code,
        interval: response.interval,
        expiresIn: response.expires_in
      });
      
      // Auth success
      setDeviceCode(null);
      setIsAuthenticating(false);
      await fetchProviders();
      
    } catch (e) {
      setError(e.toString());
      setDeviceCode(null);
      setIsAuthenticating(false);
    }
  };

  const handleCopyCode = () => {
    if (deviceCode) {
      navigator.clipboard.writeText(deviceCode.user_code);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const handleOpenBrowser = async () => {
    if (deviceCode) {
      await open(deviceCode.verification_uri);
    }
  };

  const handleSwitch = async (id) => {
    try {
      await invoke('switch_provider', { id });
      await fetchProviders();
    } catch (e) {
      setError("Failed to switch: " + e.toString());
    }
  };

  const handleDelete = async (id) => {
    if (!confirm('Are you sure you want to remove this account?')) return;
    
    try {
      await invoke('delete_provider', { id });
      await fetchProviders();
    } catch (e) {
      setError("Failed to delete: " + e.toString());
    }
  };

  // Sort providers: most recently used first
  const sortedProviders = [...providers].sort((a, b) => {
    const aTime = a.last_used_at || 0;
    const bTime = b.last_used_at || 0;
    return bTime - aTime;
  });

  const activeProvider = sortedProviders.length > 0 ? sortedProviders[0] : null;

  return (
    <div className="min-h-screen bg-slate-50 text-slate-900 font-sans p-6">
      <div className="max-w-3xl mx-auto space-y-6">
        
        {/* Header */}
        <div className="flex items-center justify-between bg-white p-6 rounded-2xl shadow-sm border border-slate-100">
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 bg-blue-600 rounded-xl flex items-center justify-center shadow-inner">
              <Github className="text-white w-7 h-7" />
            </div>
            <div>
              <h1 className="text-xl font-bold tracking-tight text-slate-900">OpenCode Switcher</h1>
              <p className="text-sm text-slate-500">Manage your GitHub Copilot identities</p>
            </div>
          </div>
          
          <button 
            onClick={handleStartAuth}
            disabled={isAuthenticating}
            className="flex items-center gap-2 bg-slate-900 hover:bg-slate-800 text-white px-4 py-2.5 rounded-lg font-medium transition-colors disabled:opacity-50"
          >
            {isAuthenticating ? <Loader2 className="w-5 h-5 animate-spin" /> : <Plus className="w-5 h-5" />}
            Add Account
          </button>
        </div>

        {/* Error Alert */}
        {error && (
          <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg text-sm flex justify-between items-center">
            <span>{error}</span>
            <button onClick={() => setError(null)} className="text-red-500 hover:text-red-700">✕</button>
          </div>
        )}

        {/* Auth Modal/Overlay */}
        {isAuthenticating && deviceCode && (
          <div className="bg-blue-50 border border-blue-200 p-6 rounded-2xl flex flex-col items-center text-center space-y-4">
            <div className="w-12 h-12 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center mb-2">
              <Loader2 className="w-6 h-6 animate-spin" />
            </div>
            <h2 className="text-lg font-semibold text-slate-900">Authorization Required</h2>
            <p className="text-sm text-slate-600 max-w-md">
              Please enter the code below at GitHub to authorize this application to use your Copilot account.
            </p>
            
            <div className="flex items-center gap-3 bg-white px-6 py-4 rounded-xl border border-slate-200 shadow-sm">
              <span className="text-3xl font-mono tracking-[0.25em] font-bold text-slate-800">
                {deviceCode.user_code}
              </span>
              <button 
                onClick={handleCopyCode}
                className="p-2 hover:bg-slate-100 rounded-lg text-slate-500 transition-colors"
                title="Copy code"
              >
                {copied ? <CheckCircle2 className="w-5 h-5 text-green-500" /> : <Copy className="w-5 h-5" />}
              </button>
            </div>
            
            <button 
              onClick={handleOpenBrowser}
              className="flex items-center gap-2 text-blue-600 font-medium hover:text-blue-700 mt-2"
            >
              Open GitHub <ExternalLink className="w-4 h-4" />
            </button>
            
            <p className="text-xs text-slate-500 mt-4">
              Waiting for authorization... This window will close automatically.
            </p>
            
            <button 
              onClick={() => {
                setIsAuthenticating(false);
                setDeviceCode(null);
              }}
              className="text-sm text-slate-500 hover:text-slate-700 underline mt-2"
            >
              Cancel
            </button>
          </div>
        )}

        {/* Accounts List */}
        <div className="space-y-4">
          <h3 className="text-sm font-semibold text-slate-400 uppercase tracking-wider pl-2">
            Your Accounts ({providers.length})
          </h3>
          
          {loading && providers.length === 0 ? (
            <div className="flex justify-center py-12">
              <RefreshCw className="w-6 h-6 text-slate-300 animate-spin" />
            </div>
          ) : providers.length === 0 && !isAuthenticating ? (
            <div className="bg-white border border-slate-200 border-dashed rounded-2xl p-12 text-center">
              <div className="w-16 h-16 bg-slate-50 rounded-full flex items-center justify-center mx-auto mb-4">
                <Github className="w-8 h-8 text-slate-400" />
              </div>
              <h3 className="text-slate-900 font-medium mb-1">No accounts found</h3>
              <p className="text-slate-500 text-sm">Add a GitHub account to start managing your Copilot identities.</p>
            </div>
          ) : (
            <div className="grid gap-4">
              {sortedProviders.map((provider) => {
                const isActive = activeProvider && activeProvider.id === provider.id;
                
                return (
                  <div 
                    key={provider.id} 
                    className={"flex items-center justify-between p-4 rounded-xl border transition-all " + (
                      isActive 
                        ? 'bg-blue-50/50 border-blue-200 shadow-sm' 
                    
