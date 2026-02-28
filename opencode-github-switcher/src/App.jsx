import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

import { Github, Plus, Trash2, CheckCircle2, Copy, ExternalLink, Loader2, RefreshCw, Globe } from 'lucide-react';
import { formatDistanceToNow } from 'date-fns';
import { enUS, zhCN, ja } from 'date-fns/locale';
import { translations, languages } from './i18n';

export default function App() {
  const [providers, setProviders] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  
  const [isAuthenticating, setIsAuthenticating] = useState(false);
  const [deviceCode, setDeviceCode] = useState(null);
  const [copied, setCopied] = useState(false);
  const [toast, setToast] = useState(null);
  
  const [currentLang, setCurrentLang] = useState(() => {
    return localStorage.getItem('app_language') || 'en';
  });

  const t = (key) => translations[currentLang]?.[key] || translations['en'][key] || key;

  const getDateLocale = () => {
    switch(currentLang) {
      case 'zh': return zhCN;
      case 'ja': return ja;
      default: return enUS;
    }
  };

  const handleLanguageChange = (e) => {
    const newLang = e.target.value;
    setCurrentLang(newLang);
    localStorage.setItem('app_language', newLang);
  };
  
  const CLIENT_ID = "Ov23li8tweQw6odWQebz";

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
      
      
      // Auto copy code
      try {
        await navigator.clipboard.writeText(response.user_code);
        setToast(t('codeCopiedToast'));
        setTimeout(() => setToast(null), 5000);
      } catch (err) {
        console.error("Failed to auto-copy code:", err);
      }
      
      // Auto open browser
      try {
        await invoke("open_url", { url: response.verification_uri });
      } catch (err) {
        console.error("Failed to auto-open browser:", err);
      }
      
      const provider = await invoke('complete_device_flow', {
        clientId: CLIENT_ID,
        deviceCode: response.device_code,
        interval: response.interval,
        expiresIn: response.expires_in
      });
      
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
      await invoke("open_url", { url: deviceCode.verification_uri });
    }
  };

  const handleSwitch = async (id) => {
    try {
      await invoke('switch_provider', { id });
      await fetchProviders();
    } catch (e) {
      setError(t('failSwitch') + e.toString());
    }
  };

  const handleDelete = async (id) => {
    if (!confirm(t('confirmRemove'))) return;
    
    try {
      await invoke('delete_provider', { id });
      await fetchProviders();
    } catch (e) {
      setError(t('failDelete') + e.toString());
    }
  };

  const sortedProviders = [...providers].sort((a, b) => {
    const aTime = a.last_used_at || 0;
    const bTime = b.last_used_at || 0;
    return bTime - aTime;
  });

  const activeProvider = sortedProviders.length > 0 ? sortedProviders[0] : null;

  return (
    <div className="min-h-screen bg-slate-50 text-slate-900 font-sans p-6 relative">
      <div className="absolute top-4 right-6 flex items-center gap-2 text-sm text-slate-600 bg-white/80 px-3 py-1.5 rounded-full border border-slate-200 shadow-sm backdrop-blur-md z-10">
        <Globe className="w-4 h-4 text-slate-400" />
        <select 
          value={currentLang} 
          onChange={handleLanguageChange}
          className="bg-transparent border-none outline-none cursor-pointer text-slate-600 font-medium hover:text-slate-900"
        >
          {languages.map(lang => (
            <option key={lang.code} value={lang.code}>{lang.label}</option>
          ))}
        </select>
      </div>

      
      <div className="max-w-3xl mx-auto space-y-6 mt-8">
        {toast && (
          <div className="fixed top-20 left-1/2 -translate-x-1/2 bg-slate-900 text-white px-6 py-3 rounded-full shadow-lg text-sm font-medium z-50 flex items-center gap-2 animate-in fade-in slide-in-from-top-4 duration-300">
            <CheckCircle2 className="w-5 h-5 text-green-400" />
            {toast}
          </div>
        )}
        <div className="flex items-center justify-between bg-white p-6 rounded-2xl shadow-sm border border-slate-100">
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 bg-blue-600 rounded-xl flex items-center justify-center shadow-inner">
              <Github className="text-white w-7 h-7" />
            </div>
            <div>
              <h1 className="text-xl font-bold tracking-tight text-slate-900">{t('appTitle')}</h1>
              <p className="text-sm text-slate-500">{t('appSubtitle')}</p>
            </div>
          </div>
          
          <button 
            onClick={handleStartAuth}
            disabled={isAuthenticating}
            className="flex items-center gap-2 bg-slate-900 hover:bg-slate-800 text-white px-4 py-2.5 rounded-lg font-medium transition-colors disabled:opacity-50"
          >
            {isAuthenticating ? <Loader2 className="w-5 h-5 animate-spin" /> : <Plus className="w-5 h-5" />}
            {t('addAccount')}
          </button>
        </div>

        {error && (
          <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg text-sm flex justify-between items-center">
            <span>{error}</span>
            <button onClick={() => setError(null)} className="text-red-500 hover:text-red-700">✕</button>
          </div>
        )}

        {isAuthenticating && deviceCode && (
          <div className="bg-blue-50 border border-blue-200 p-6 rounded-2xl flex flex-col items-center text-center space-y-4 relative">
            <div className="w-12 h-12 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center mb-2">
              <Loader2 className="w-6 h-6 animate-spin" />
            </div>
            <h2 className="text-lg font-semibold text-slate-900">{t('authRequired')}</h2>
            <p className="text-sm text-slate-600 max-w-md">
              {t('authDesc')}
            </p>
            
            <div className="flex items-center gap-3 bg-white px-6 py-4 rounded-xl border border-slate-200 shadow-sm">
              <span className="text-3xl font-mono tracking-[0.25em] font-bold text-slate-800">
                {deviceCode.user_code}
              </span>
              <button 
                onClick={handleCopyCode}
                className="p-2 hover:bg-slate-100 rounded-lg text-slate-500 transition-colors"
                title={t('copyCode')}
              >
                {copied ? <CheckCircle2 className="w-5 h-5 text-green-500" /> : <Copy className="w-5 h-5" />}
              </button>
            </div>
            
            <button 
              onClick={handleOpenBrowser}
              className="flex items-center gap-2 text-blue-600 font-medium hover:text-blue-700 mt-2"
            >
              {t('openGithub')} <ExternalLink className="w-4 h-4" />
            </button>
            
            <p className="text-xs text-slate-500 mt-4">
              {t('waitingAuth')}
            </p>
            
            <button 
              onClick={() => {
                setIsAuthenticating(false);
                setDeviceCode(null);
              }}
              className="text-sm text-slate-500 hover:text-slate-700 underline mt-2"
            >
              {t('cancel')}
            </button>
          </div>
        )}

        <div className="space-y-4">
          <h3 className="text-sm font-semibold text-slate-400 uppercase tracking-wider pl-2">
            {t('yourAccounts')} ({providers.length})
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
              <h3 className="text-slate-900 font-medium">{t('noAccounts')}</h3>
              <p className="text-slate-500 text-sm mt-1">{t('clickAddAccount')}</p>
            </div>
          ) : (
            <div className="grid gap-4">
              {sortedProviders.map((provider) => {
                const isActive = activeProvider && activeProvider.id === provider.id;
                
                return (
                  <div 
                    key={provider.id} 
                    className={`flex items-center justify-between p-5 rounded-2xl border transition-all ${
                      isActive 
                        ? 'bg-white border-blue-200 ring-1 ring-blue-100 shadow-sm' 
                        : 'bg-white border-slate-200 hover:border-slate-300'
                    }`}
                  >
                    <div className="flex items-center gap-4">
                      <div className={`w-10 h-10 rounded-full flex items-center justify-center overflow-hidden ${
                        isActive ? 'bg-blue-50 text-blue-600' : 'bg-slate-50 text-slate-400'
                      }`}>
                        {provider.avatar_url ? (
                          <img src={provider.avatar_url} alt={provider.name} className="w-full h-full object-cover" />
                        ) : (
                          <Github className="w-5 h-5" />
                        )}
                      </div>
                      <div>
                        <div className="flex items-center gap-2">
                          <span className="font-semibold text-slate-900">
                            {provider.name || 'Unknown User'}
                          </span>
                          {isActive && (
                            <span className="bg-blue-100 text-blue-700 text-[10px] px-2 py-0.5 rounded-full font-medium uppercase tracking-wide">
                              {t('active')}
                            </span>
                          )}
                        </div>
                        <div className="text-xs text-slate-500 mt-1 flex items-center gap-2">
                          <span>{t('added')}: {provider.created_at ? new Date(provider.created_at * 1000).toLocaleDateString() : 'Unknown'}</span>
                          {provider.last_used_at && (
                            <>
                              <span className="w-1 h-1 bg-slate-300 rounded-full"></span>
                              <span>{t('lastUsed')}: {formatDistanceToNow(provider.last_used_at * 1000, { addSuffix: true, locale: getDateLocale() })}</span>
                            </>
                          )}
                        </div>
                      </div>
                    </div>
                    
                    <div className="flex items-center gap-3">
                      {!isActive && (
                        <button
                          onClick={() => handleSwitch(provider.id)}
                          className="px-4 py-2 bg-slate-100 hover:bg-slate-200 text-slate-700 text-sm font-medium rounded-lg transition-colors"
                        >
                          {t('switchTo')}
                        </button>
                      )}
                      
                      <button
                        onClick={() => handleDelete(provider.id)}
                        className="p-2 text-slate-400 hover:text-red-500 hover:bg-red-50 rounded-lg transition-colors"
                        title={t('removeAccount')}
                      >
                        <Trash2 className="w-5 h-5" />
                      </button>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
