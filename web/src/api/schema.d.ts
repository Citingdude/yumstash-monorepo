/** Starter contract. Regenerate from the running API with pnpm api:generate. */
export interface paths {
  '/api/health': {
    get: {
      responses: {
        200: { content: { 'application/json': components['schemas']['HealthResponse'] } }
      }
    }
  }
}
export interface components {
  schemas: { HealthResponse: { status: string } }
}
