# Coolify Deployment Guide

## Environment Setup

1. Copy the environment variables from `coolify-env-template.txt` to your Coolify deployment
2. Update the values according to your existing PostgreSQL and Redis instances
3. Set your domain in `BASE_URL` and `NEXT_PUBLIC_API_URL`

## Selective Service Deployment

Since PostgreSQL and Redis are already running, the docker-compose.yml excludes them. 
For selective deployments in Coolify, use these approaches:

### Deploy Only Backend
```bash
docker-compose up -d --build backend
```

### Deploy Only Frontend  
```bash
docker-compose up -d --build frontend
```

### Deploy Only Monitoring Stack
```bash
docker-compose up -d prometheus grafana redis-exporter postgres-exporter
```

### Deploy All Services
```bash
docker-compose up -d --build
```

## Service Dependencies

The compose file includes proper health checks and dependencies:
- Frontend waits for backend to be healthy
- Prometheus waits for backend to be healthy  
- Grafana waits for Prometheus to start

## Resource Limits

Production-optimized resource limits are configured:
- Backend: 256M limit, 128M reservation
- Frontend: 512M limit, 256M reservation  
- Prometheus: 512M limit, 256M reservation
- Grafana: 256M limit, 128M reservation
- Exporters: 64M limit, 32M reservation

## Health Checks

All services include health checks for proper deployment verification:
- Backend: `/health` endpoint check
- Frontend: Root endpoint check
- Other services: Default container health checks

## Coolify Integration

This setup is optimized for Coolify with:
- Environment variable templating
- Health check integration
- Resource management
- Selective deployment support
- Exclude existing databases (postgres/redis) 