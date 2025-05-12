import { useState } from "react";
import { PageContent } from "./components/PageContent";
import "./App.css";

type Page = 'setup' | 'cleanup' | 'settings' | 'about';

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('setup');

  return (
    <main className="container">
      <div className="flex h-screen">
        {/* サイドバー */}
        <div className="w-64 bg-slate-800 text-white p-4">
          <h2 className="text-xl font-bold mb-6">Stream Settings</h2>
          <nav className="space-y-2">
            <button
              onClick={() => setCurrentPage('setup')}
              className={`w-full flex items-center p-2 rounded hover:bg-slate-700 ${currentPage === 'setup' ? 'bg-slate-700' : ''}`}
            >
              <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
                <path d="M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V17a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-6.586l.293.293a1 1 0 001.414-1.414l-7-7z" />
              </svg>
              Setup
            </button>
            <button
              onClick={() => setCurrentPage('cleanup')}
              className={`w-full flex items-center p-2 rounded hover:bg-slate-700 ${currentPage === 'cleanup' ? 'bg-slate-700' : ''}`}
            >
              <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
                <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z" clipRule="evenodd" />
              </svg>
              Cleanup
            </button>
            <button
              onClick={() => setCurrentPage('settings')}
              className={`w-full flex items-center p-2 rounded hover:bg-slate-700 ${currentPage === 'settings' ? 'bg-slate-700' : ''}`}
            >
              <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
                <path fillRule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clipRule="evenodd" />
              </svg>
              Settings
            </button>
            <button
              onClick={() => setCurrentPage('about')}
              className={`w-full flex items-center p-2 rounded hover:bg-slate-700 ${currentPage === 'about' ? 'bg-slate-700' : ''}`}
            >
              <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
                <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
              </svg>
              About
            </button>
          </nav>
        </div>

        {/* メインコンテンツ */}
        <div className="flex-1 p-8 bg-slate-50">
          <PageContent currentPage={currentPage} />
        </div>
      </div>
    </main>
  );
}

export default App;
