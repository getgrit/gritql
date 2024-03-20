import { Cypress } from 'local-cypress';

interface RouteResponses {
  'get-survey': object;
  response: object;
  unsubscribe: object;
}

declare global {
  namespace Cypress {
    type RouteVariant = `${keyof RouteResponses}:${string}`;
    type RouteResponse<TRoute extends RouteVariant> =
      TRoute extends `${infer Key extends keyof RouteResponses}:${string}`
        ? RouteResponses[Key]
        : never;
    interface Chainable {
      getRouteVariant<TRoute extends RouteVariant>(
        routeVariant: TRoute,
      ): Chainable<RouteResponse<TRoute>>;
      /**
       * @alias cy.mocksUseRouteVariant
       * @param routeVariant
       */
      useDataRouteVariant(routeVariant: RouteVariant): void;
      getApiRequests(): Chainable<any[]>;
      getApiSubmits(): Chainable<object[]>;
      getApiLoads(): Chainable<object[]>;
    }
  }
}

Cypress.Commands.add('getRouteVariant', (routeVariant) =>
  cy
    .request<{
      preview: {
        body: RouteResponses[typeof routeVariant extends `${infer T}:${string}`
          ? T
          : never];
      };
    }>('GET', `http://localhost:3110/api/mock/variants/${routeVariant}`)
    .then((res) => res.body.preview.body),
);