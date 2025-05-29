import type { Metadata } from "next";
import localFont from "next/font/local";
import "./globals.css";
import { Toaster } from "sonner";

const geistSans = localFont({
  src: "./fonts/GeistVF.woff2",
  variable: "--font-geist-sans",
  weight: "100 900",
});
const geistMono = localFont({
  src: "./fonts/GeistMonoVF.woff2",
  variable: "--font-geist-mono",
  weight: "100 900",
});

export const metadata: Metadata = {
  title: "LinkShort - Fast Link Shortener",
  description: "Create short, memorable links in seconds. Track clicks, analyze traffic, and manage all your links in one place.",
  keywords: "link shortener, url shortener, short links, analytics",
  openGraph: {
    title: "LinkShort - Fast Link Shortener",
    description: "Create short, memorable links in seconds",
    type: "website",
  },
  robots: {
    index: true,
    follow: true,
  },
};

export function reportWebVitals(metric: any) {
  if (metric.label === 'web-vital') {
    if (process.env.NODE_ENV === 'production') {
      fetch('/api/analytics/web-vitals', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(metric),
      }).catch(() => {});
    } else {
      console.log(metric);
    }
  }
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <head>
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin="" />
        <link rel="dns-prefetch" href="//link.aescipher.xyz" />
      </head>
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
      >
        {children}
        <Toaster position="top-center" richColors />
      </body>
    </html>
  );
}
