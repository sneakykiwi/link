import axios from 'axios';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

export interface CreateLinkRequest {
  url: string;
  customCode?: string;
  expiresInHours?: number;
}

export interface CreateLinkResponse {
  shortUrl: string;
  shortCode: string;
}

export const api = {
  createLink: async (data: CreateLinkRequest): Promise<CreateLinkResponse> => {
    const response = await axios.post(`${API_BASE_URL}/api/shorten`, data);
    return response.data;
  },

  getLink: async (shortCode: string) => {
    const response = await axios.get(`${API_BASE_URL}/api/links/${shortCode}`);
    return response.data;
  },

  getAnalytics: async (shortCode: string) => {
    const response = await axios.get(`${API_BASE_URL}/api/analytics/${shortCode}`);
    return response.data;
  },
}; 