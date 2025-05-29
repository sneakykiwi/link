"use client";

import useSWR from 'swr';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { BarChart3, Eye, MousePointer, Calendar, Globe } from 'lucide-react';

interface AnalyticsData {
  totalClicks: number;
  uniqueClicks: number;
  clicksByDate: { date: string; clicks: number }[];
  clicksByCountry: { country: string; clicks: number }[];
  clicksByReferrer: { referrer: string; clicks: number }[];
  topLinks: {
    shortCode: string;
    originalUrl: string;
    clicks: number;
  }[];
}

const fetcher = (url: string) => fetch(url).then((res) => res.json());

interface AnalyticsProps {
  shortCode?: string;
}

export function Analytics({ shortCode }: AnalyticsProps) {
  const endpoint = shortCode ? `/api/analytics/${shortCode}` : '/api/analytics';
  const { data: analytics, error, isLoading } = useSWR<AnalyticsData>(endpoint, fetcher);

  if (isLoading) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center space-x-2">
            <BarChart3 className="h-5 w-5" />
            <span>Analytics</span>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-center py-8">Loading analytics...</div>
        </CardContent>
      </Card>
    );
  }

  if (error) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center space-x-2">
            <BarChart3 className="h-5 w-5" />
            <span>Analytics</span>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-center py-8 text-red-500">
            Failed to load analytics. Please try again.
          </div>
        </CardContent>
      </Card>
    );
  }

  if (!analytics) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center space-x-2">
            <BarChart3 className="h-5 w-5" />
            <span>Analytics</span>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-center py-8 text-muted-foreground">
            No analytics data available yet.
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <Card>
          <CardContent className="p-6">
            <div className="flex items-center space-x-2">
              <Eye className="h-4 w-4 text-muted-foreground" />
              <div className="text-sm font-medium text-muted-foreground">Total Clicks</div>
            </div>
            <div className="text-2xl font-bold">{analytics.totalClicks.toLocaleString()}</div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-6">
            <div className="flex items-center space-x-2">
              <MousePointer className="h-4 w-4 text-muted-foreground" />
              <div className="text-sm font-medium text-muted-foreground">Unique Clicks</div>
            </div>
            <div className="text-2xl font-bold">{analytics.uniqueClicks.toLocaleString()}</div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-6">
            <div className="flex items-center space-x-2">
              <BarChart3 className="h-4 w-4 text-muted-foreground" />
              <div className="text-sm font-medium text-muted-foreground">Click Rate</div>
            </div>
            <div className="text-2xl font-bold">
              {analytics.totalClicks > 0 
                ? `${((analytics.uniqueClicks / analytics.totalClicks) * 100).toFixed(1)}%`
                : '0%'
              }
            </div>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Detailed Analytics</CardTitle>
        </CardHeader>
        <CardContent>
          <Tabs defaultValue="geography" className="w-full">
            <TabsList className="grid w-full grid-cols-3">
              <TabsTrigger value="geography">Geography</TabsTrigger>
              <TabsTrigger value="referrers">Referrers</TabsTrigger>
              <TabsTrigger value="toplinks">Top Links</TabsTrigger>
            </TabsList>

            <TabsContent value="geography" className="space-y-4">
              <div className="space-y-2">
                <h4 className="text-sm font-medium text-muted-foreground flex items-center space-x-2">
                  <Globe className="h-4 w-4" />
                  <span>Clicks by Country</span>
                </h4>
                {analytics.clicksByCountry.length > 0 ? (
                  <div className="space-y-2">
                    {analytics.clicksByCountry.slice(0, 5).map((item, index) => (
                      <div key={item.country} className="flex items-center justify-between">
                        <span className="text-sm">{item.country}</span>
                        <div className="flex items-center space-x-2">
                          <div className="w-24 bg-muted h-2 rounded-full overflow-hidden">
                            <div 
                              className="h-full bg-primary rounded-full"
                              style={{
                                width: `${(item.clicks / analytics.clicksByCountry[0].clicks) * 100}%`
                              }}
                            />
                          </div>
                          <span className="text-sm font-medium">{item.clicks}</span>
                        </div>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="text-sm text-muted-foreground">No geography data available</div>
                )}
              </div>
            </TabsContent>

            <TabsContent value="referrers" className="space-y-4">
              <div className="space-y-2">
                <h4 className="text-sm font-medium text-muted-foreground">Traffic Sources</h4>
                {analytics.clicksByReferrer.length > 0 ? (
                  <div className="space-y-2">
                    {analytics.clicksByReferrer.slice(0, 5).map((item, index) => (
                      <div key={item.referrer} className="flex items-center justify-between">
                        <span className="text-sm truncate max-w-32">
                          {item.referrer || 'Direct'}
                        </span>
                        <div className="flex items-center space-x-2">
                          <div className="w-24 bg-muted h-2 rounded-full overflow-hidden">
                            <div 
                              className="h-full bg-primary rounded-full"
                              style={{
                                width: `${(item.clicks / analytics.clicksByReferrer[0].clicks) * 100}%`
                              }}
                            />
                          </div>
                          <span className="text-sm font-medium">{item.clicks}</span>
                        </div>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="text-sm text-muted-foreground">No referrer data available</div>
                )}
              </div>
            </TabsContent>

            <TabsContent value="toplinks" className="space-y-4">
              <div className="space-y-2">
                <h4 className="text-sm font-medium text-muted-foreground">Most Clicked Links</h4>
                {analytics.topLinks.length > 0 ? (
                  <div className="space-y-3">
                    {analytics.topLinks.slice(0, 5).map((link, index) => (
                      <div key={link.shortCode} className="flex items-center justify-between p-3 border rounded-lg">
                        <div className="space-y-1">
                          <div className="flex items-center space-x-2">
                            <Badge variant="outline">{link.shortCode}</Badge>
                            <span className="text-xs text-muted-foreground">#{index + 1}</span>
                          </div>
                          <div className="text-sm text-muted-foreground truncate max-w-64">
                            {link.originalUrl}
                          </div>
                        </div>
                        <div className="text-right">
                          <div className="text-lg font-semibold">{link.clicks}</div>
                          <div className="text-xs text-muted-foreground">clicks</div>
                        </div>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="text-sm text-muted-foreground">No links data available</div>
                )}
              </div>
            </TabsContent>
          </Tabs>
        </CardContent>
      </Card>
    </div>
  );
} 