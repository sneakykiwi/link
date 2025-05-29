"use client";

import { useState } from 'react';
import useSWR from 'swr';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { toast } from 'sonner';
import { copyToClipboard, formatDate } from '@/lib/utils';
import { Copy, ExternalLink, Eye } from 'lucide-react';

interface Link {
  id: string;
  shortCode: string;
  originalUrl: string;
  shortUrl: string;
  clicks: number;
  createdAt: string;
  expiresAt?: string;
}

const fetcher = (url: string) => fetch(url).then((res) => res.json());

export function LinkList() {
  const { data: links, error, isLoading } = useSWR<Link[]>('/api/links', fetcher);

  const handleCopy = async (url: string) => {
    try {
      await copyToClipboard(url);
      toast.success('Link copied to clipboard!');
    } catch (error) {
      toast.error('Failed to copy link');
    }
  };

  const openLink = (url: string) => {
    window.open(url, '_blank');
  };

  if (isLoading) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Your Links</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-center py-8">Loading your links...</div>
        </CardContent>
      </Card>
    );
  }

  if (error) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Your Links</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-center py-8 text-red-500">
            Failed to load links. Please try again.
          </div>
        </CardContent>
      </Card>
    );
  }

  if (!links || links.length === 0) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Your Links</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-center py-8 text-muted-foreground">
            No links created yet. Create your first short link above!
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Your Links</CardTitle>
      </CardHeader>
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Short Link</TableHead>
              <TableHead>Original URL</TableHead>
              <TableHead>Clicks</TableHead>
              <TableHead>Created</TableHead>
              <TableHead>Status</TableHead>
              <TableHead>Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {links.map((link) => {
              const isExpired = link.expiresAt && new Date(link.expiresAt) < new Date();
              
              return (
                <TableRow key={link.id}>
                  <TableCell>
                    <div className="flex items-center space-x-2">
                      <code className="text-sm bg-muted px-2 py-1 rounded">
                        {link.shortCode}
                      </code>
                    </div>
                  </TableCell>
                  <TableCell>
                    <div className="max-w-xs truncate" title={link.originalUrl}>
                      {link.originalUrl}
                    </div>
                  </TableCell>
                  <TableCell>
                    <div className="flex items-center space-x-1">
                      <Eye className="h-4 w-4" />
                      <span>{link.clicks}</span>
                    </div>
                  </TableCell>
                  <TableCell>
                    <div className="text-sm text-muted-foreground">
                      {formatDate(new Date(link.createdAt))}
                    </div>
                  </TableCell>
                  <TableCell>
                    <Badge variant={isExpired ? "destructive" : "default"}>
                      {isExpired ? "Expired" : "Active"}
                    </Badge>
                  </TableCell>
                  <TableCell>
                    <div className="flex items-center space-x-2">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleCopy(link.shortUrl)}
                      >
                        <Copy className="h-4 w-4" />
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => openLink(link.shortUrl)}
                      >
                        <ExternalLink className="h-4 w-4" />
                      </Button>
                    </div>
                  </TableCell>
                </TableRow>
              );
            })}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
} 