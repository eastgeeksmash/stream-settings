import React from 'react';
import { Page } from '../types/page';
import { SetupPage } from './pages/SetupPage';
import { CleanupPage } from './pages/CleanupPage';
import { SettingsPage } from './pages/SettingsPage';
import { AboutPage } from './pages/AboutPage';

interface PageContentProps {
  currentPage: Page;
}

export const PageContent: React.FC<PageContentProps> = ({ currentPage }) => {
  switch (currentPage) {
    case 'setup':
      return <SetupPage />;
    case 'cleanup':
      return <CleanupPage />;
    case 'settings':
      return <SettingsPage />;
    case 'about':
      return <AboutPage />;
  }
}; 