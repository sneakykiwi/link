import { NextRequest, NextResponse } from 'next/server';
import axios from 'axios';

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    
    const response = await axios.post(
      `${process.env.NEXT_PUBLIC_API_URL}/api/shorten`,
      body
    );
    
    return NextResponse.json(response.data);
  } catch (error) {
    console.error('Error shortening URL:', error);
    return NextResponse.json(
      { error: 'Failed to shorten URL' },
      { status: 500 }
    );
  }
} 