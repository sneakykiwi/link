#!/bin/bash

set -e

BACKEND_CHANGED=false
FRONTEND_CHANGED=false
MONITORING_CHANGED=false

if [ "$1" = "backend" ] || [ "$1" = "all" ]; then
    BACKEND_CHANGED=true
fi

if [ "$1" = "frontend" ] || [ "$1" = "all" ]; then
    FRONTEND_CHANGED=true
fi

if [ "$1" = "monitoring" ] || [ "$1" = "all" ]; then
    MONITORING_CHANGED=true
fi

echo "Starting selective deployment..."

if [ "$BACKEND_CHANGED" = true ]; then
    echo "Rebuilding backend service..."
    docker-compose build backend
    docker-compose up -d backend
    echo "Backend deployment complete."
fi

if [ "$FRONTEND_CHANGED" = true ]; then
    echo "Rebuilding frontend service..."
    docker-compose build frontend
    docker-compose up -d frontend
    echo "Frontend deployment complete."
fi

if [ "$MONITORING_CHANGED" = true ]; then
    echo "Updating monitoring services..."
    docker-compose up -d prometheus grafana redis-exporter postgres-exporter
    echo "Monitoring deployment complete."
fi

if [ "$1" = "" ]; then
    echo "Usage: ./deploy.sh [backend|frontend|monitoring|all]"
    echo "  backend    - Deploy only backend service"
    echo "  frontend   - Deploy only frontend service"
    echo "  monitoring - Deploy only monitoring services"
    echo "  all        - Deploy all services"
fi

echo "Deployment finished successfully!" 