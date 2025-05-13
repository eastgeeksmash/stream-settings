import { useState } from "react";
import { PageContent } from "./components/PageContent";
import "./App.css";
import { Toaster } from "@/components/ui/sonner"
import { ThemeProvider } from "./components/theme-provider"
import {
  Sidebar,
  SidebarHeader,
  SidebarContent,
  SidebarNav,
  SidebarNavItem,
} from "@/components/ui/sidebar"
import {
  Rocket,
  Trash2,
  Settings,
  Info,
} from "lucide-react"

type Page = 'setup' | 'cleanup' | 'settings' | 'about';

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('setup');

  return (
    <ThemeProvider defaultTheme="system" storageKey="stream-settings-theme">
      <main className="container">
        <Toaster />
        
        <div className="flex h-screen">
          <Sidebar>
            <SidebarHeader>
              <h2 className="text-xl font-bold">Stream Settings</h2>
            </SidebarHeader>
            <SidebarContent>
              <SidebarNav>
                <SidebarNavItem
                  active={currentPage === 'setup'}
                  onClick={() => setCurrentPage('setup')}
                >
                  <Rocket className="mr-2 h-4 w-4" />
                  Setup
                </SidebarNavItem>
                <SidebarNavItem
                  active={currentPage === 'cleanup'}
                  onClick={() => setCurrentPage('cleanup')}
                >
                  <Trash2 className="mr-2 h-4 w-4" />
                  Cleanup
                </SidebarNavItem>
                <SidebarNavItem
                  active={currentPage === 'settings'}
                  onClick={() => setCurrentPage('settings')}
                >
                  <Settings className="mr-2 h-4 w-4" />
                  Settings
                </SidebarNavItem>
                <SidebarNavItem
                  active={currentPage === 'about'}
                  onClick={() => setCurrentPage('about')}
                >
                  <Info className="mr-2 h-4 w-4" />
                  About
                </SidebarNavItem>
              </SidebarNav>
            </SidebarContent>
          </Sidebar>

          {/* メインコンテンツ */}
          <div className="flex-1 p-8 bg-background">
            <PageContent currentPage={currentPage} />
          </div>
        </div>
      </main>
    </ThemeProvider>
  );
}

export default App;
