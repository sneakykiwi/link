import { LinkForm } from './components/LinkForm';
import { LinkList } from './components/LinkList';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Link, BarChart3, List } from 'lucide-react';
import dynamic from 'next/dynamic';

const Analytics = dynamic(() => import('./components/Analytics').then(mod => ({ default: mod.Analytics })), {
  loading: () => (
    <div className="flex items-center justify-center p-8">
      <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      <span className="ml-2 text-muted-foreground">Loading analytics...</span>
    </div>
  ),
  ssr: true,
});

export default function Home() {
  return (
    <main className="min-h-screen bg-gradient-to-br from-background to-muted/30">
      <div className="container mx-auto px-4 py-8 max-w-6xl">
        <div className="text-center space-y-4 mb-12">
          <div className="flex items-center justify-center space-x-2 mb-4">
            <Link className="h-8 w-8 text-primary" />
            <h1 className="text-4xl font-bold tracking-tight">LinkShort</h1>
          </div>
          <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
            Create short, memorable links in seconds. Track clicks, analyze traffic, and manage all your links in one place.
          </p>
        </div>

        <div className="space-y-8">
          <div className="max-w-2xl mx-auto">
            <LinkForm />
          </div>

          <Tabs defaultValue="links" className="w-full">
            <TabsList className="grid w-full grid-cols-2 max-w-md mx-auto">
              <TabsTrigger value="links" className="flex items-center space-x-2">
                <List className="h-4 w-4" />
                <span>My Links</span>
              </TabsTrigger>
              <TabsTrigger value="analytics" className="flex items-center space-x-2">
                <BarChart3 className="h-4 w-4" />
                <span>Analytics</span>
              </TabsTrigger>
            </TabsList>

            <div className="mt-8">
              <TabsContent value="links">
                <LinkList />
              </TabsContent>

              <TabsContent value="analytics">
                <Analytics />
              </TabsContent>
            </div>
          </Tabs>
        </div>

        <footer className="mt-16 text-center text-sm text-muted-foreground">
          <p>Built with ❤️ using Next.js, React Hook Form, and shadcn/ui</p>
        </footer>
      </div>
    </main>
  );
}
