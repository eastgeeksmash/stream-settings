import { useState } from "react";
import { PageContent } from "./components/PageContent";
import "./App.css";
import { Toaster } from "@/components/ui/sonner"
import { TooltipProvider } from "@/components/ui/tooltip"
import { AdminWarning } from "./components/admin-warning"
import { StartupUpdateCheck } from "./components/startup-update-check"
import { ThemeProvider } from "./components/theme-provider"
import {
  Sidebar,
  SidebarHeader,
  SidebarContent,
  SidebarNav,
  SidebarNavItem,
} from "@/components/ui/sidebar"
import Rocket from "lucide-react/dist/esm/icons/rocket";
import Network from "lucide-react/dist/esm/icons/network";
import Trash2 from "lucide-react/dist/esm/icons/trash-2";
import Package from "lucide-react/dist/esm/icons/package";
import Settings from "lucide-react/dist/esm/icons/settings";
import Info from "lucide-react/dist/esm/icons/info";

const setupNavIcon = <Rocket className="mr-2 h-4 w-4" />;
const networkNavIcon = <Network className="mr-2 h-4 w-4" />;
const cleanupNavIcon = <Trash2 className="mr-2 h-4 w-4" />;
const installNavIcon = <Package className="mr-2 h-4 w-4" />;
const settingsNavIcon = <Settings className="mr-2 h-4 w-4" />;
const aboutNavIcon = <Info className="mr-2 h-4 w-4" />;

type Page = 'setup' | 'network' | 'cleanup' | 'install' | 'settings' | 'about';

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('setup');

  return (
    <ThemeProvider defaultTheme="system" storageKey="stream-settings-theme">
      <TooltipProvider>
        <main className="container">
          <Toaster />
          <StartupUpdateCheck />
          <AdminWarning />

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
                    {setupNavIcon}
                    Setup
                  </SidebarNavItem>
                  <SidebarNavItem
                    active={currentPage === 'network'}
                    onClick={() => setCurrentPage('network')}
                  >
                    {networkNavIcon}
                    Network
                  </SidebarNavItem>
                  <SidebarNavItem
                    active={currentPage === 'cleanup'}
                    onClick={() => setCurrentPage('cleanup')}
                  >
                    {cleanupNavIcon}
                    Cleanup
                  </SidebarNavItem>
                  <SidebarNavItem
                    active={currentPage === 'install'}
                    onClick={() => setCurrentPage('install')}
                  >
                    {installNavIcon}
                    Install
                  </SidebarNavItem>
                  <SidebarNavItem
                    active={currentPage === 'settings'}
                    onClick={() => setCurrentPage('settings')}
                  >
                    {settingsNavIcon}
                    Settings
                  </SidebarNavItem>
                  <SidebarNavItem
                    active={currentPage === 'about'}
                    onClick={() => setCurrentPage('about')}
                  >
                    {aboutNavIcon}
                    About
                  </SidebarNavItem>
                </SidebarNav>
              </SidebarContent>
            </Sidebar>

            <div className="flex-1 min-h-0 overflow-y-auto p-8 bg-background">
              <PageContent currentPage={currentPage} />
            </div>
          </div>
        </main>
      </TooltipProvider>
    </ThemeProvider>
  );
}

export default App;
