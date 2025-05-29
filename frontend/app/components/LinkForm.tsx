"use client";

import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { toast } from 'sonner';
import { api, CreateLinkResponse } from '@/lib/api';
import { isValidUrl, copyToClipboard } from '@/lib/utils';

const linkSchema = z.object({
  url: z.string().url('Please enter a valid URL'),
  customCode: z.string().optional(),
  expiresInHours: z.number().optional(),
});

type LinkFormData = z.infer<typeof linkSchema>;

interface LinkFormProps {
  onLinkCreated?: (link: CreateLinkResponse) => void;
}

export function LinkForm({ onLinkCreated }: LinkFormProps) {
  const [isLoading, setIsLoading] = useState(false);
  const [createdLink, setCreatedLink] = useState<CreateLinkResponse | null>(null);

  const form = useForm<LinkFormData>({
    resolver: zodResolver(linkSchema),
    defaultValues: {
      url: '',
      customCode: '',
    },
  });

  const onSubmit = async (data: LinkFormData) => {
    setIsLoading(true);
    try {
      const response = await api.createLink(data);
      setCreatedLink(response);
      onLinkCreated?.(response);
      toast.success('Link created successfully!');
      form.reset();
    } catch (error) {
      toast.error('Failed to create link. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleCopy = async (url: string) => {
    try {
      await copyToClipboard(url);
      toast.success('Link copied to clipboard!');
    } catch (error) {
      toast.error('Failed to copy link');
    }
  };

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Create Short Link</CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
            <div>
              <Input
                {...form.register('url')}
                placeholder="Enter your long URL here..."
                className="w-full"
              />
              {form.formState.errors.url && (
                <p className="text-sm text-red-500 mt-1">
                  {form.formState.errors.url.message}
                </p>
              )}
            </div>

            <div>
              <Input
                {...form.register('customCode')}
                placeholder="Custom short code (optional)"
                className="w-full"
              />
              {form.formState.errors.customCode && (
                <p className="text-sm text-red-500 mt-1">
                  {form.formState.errors.customCode.message}
                </p>
              )}
            </div>

            <Button type="submit" disabled={isLoading} className="w-full">
              {isLoading ? 'Creating...' : 'Create Short Link'}
            </Button>
          </form>
        </CardContent>
      </Card>

      {createdLink && (
        <Card>
          <CardHeader>
            <CardTitle>Your Short Link</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="flex items-center space-x-2">
              <Input
                value={createdLink.shortUrl}
                readOnly
                className="flex-1"
              />
              <Button
                onClick={() => handleCopy(createdLink.shortUrl)}
                variant="outline"
              >
                Copy
              </Button>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
} 