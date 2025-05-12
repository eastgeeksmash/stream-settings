import { useState } from "react";
import { PageContent } from "./components/PageContent";
import "./App.css";
import { Toaster } from "@/components/ui/sonner"

type Page = 'setup' | 'cleanup' | 'settings' | 'about';

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('setup');

  return (
    <main className="container">
      <Toaster />
      
      <div className="flex h-screen">
        {/* サイドバー */}
        <div className="w-64 bg-slate-800 text-white p-4">
          <h2 className="text-xl font-bold mb-6">Stream Settings</h2>
          <nav className="space-y-2">
            <button
              type="button"
              onClick={() => setCurrentPage('setup')}
              className={`w-full flex items-center p-2 rounded hover:bg-slate-700 ${currentPage === 'setup' ? 'bg-slate-700' : ''}`}
            >
              <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
                <path d="M12 12V3" />
                <path d="M19 10a7 7 0 0 0-14 0" />
                <path d="M12 21v-3" />
                <path d="M12 17a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z" />
              </svg>
              Setup
            </button>
            <button
              type="button"
              onClick={() => setCurrentPage('cleanup')}
              className={`w-full flex items-center p-2 rounded hover:bg-slate-700 ${currentPage === 'cleanup' ? 'bg-slate-700' : ''}`}
            >
              <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
                <path d="M3 6h18" />
                <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" />
                <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
                <line x1="10" y1="11" x2="10" y2="17" />
                <line x1="14" y1="11" x2="14" y2="17" />
              </svg>
              Cleanup
            </button>
            <button
              type="button"
              onClick={() => setCurrentPage('settings')}
              className={`w-full flex items-center p-2 rounded hover:bg-slate-700 ${currentPage === 'settings' ? 'bg-slate-700' : ''}`}
            >
              <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
                <circle cx="12" cy="12" r="3" />
                <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
              </svg>
              Settings
            </button>
            <button
              type="button"
              onClick={() => setCurrentPage('about')}
              className={`w-full flex items-center p-2 rounded hover:bg-slate-700 ${currentPage === 'about' ? 'bg-slate-700' : ''}`}
            >
              <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
                <circle cx="12" cy="12" r="10" />
                <line x1="12" y1="16" x2="12" y2="12" />
                <line x1="12" y1="8" x2="12.01" y2="8" />
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
