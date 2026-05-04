/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ApiKey } from '../models/ApiKey';
import type { CreateApiKeyRequest } from '../models/CreateApiKeyRequest';
import type { CreateApiKeyResponse } from '../models/CreateApiKeyResponse';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class ApiKeyService {
    /**
     * List the current user's API keys
     * Returns all active API keys belonging to the authenticated user. Requires JWT authentication; API-key auth is rejected here.
     * @returns ApiKey Successful operation
     * @throws ApiError
     */
    public static listApiKeys(): CancelablePromise<Array<ApiKey>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/api-keys',
            errors: {
                401: `Unauthorized`,
                403: `API-key auth used; JWT required`,
            },
        });
    }
    /**
     * Create a new API key
     * Creates a new API key for the authenticated user. The plaintext token is only returned in this response and cannot be retrieved later.
     * @param body
     * @returns CreateApiKeyResponse API key created
     * @throws ApiError
     */
    public static createApiKey(
        body: CreateApiKeyRequest,
    ): CancelablePromise<CreateApiKeyResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api-keys',
            body: body,
            errors: {
                400: `Invalid input`,
                401: `Unauthorized`,
                403: `API-key auth used; JWT required`,
            },
        });
    }
    /**
     * Revoke an API key
     * Marks the API key as revoked so it can no longer authenticate.
     * @param id
     * @returns void
     * @throws ApiError
     */
    public static revokeApiKey(
        id: number,
    ): CancelablePromise<void> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/api-keys/{id}',
            path: {
                'id': id,
            },
            errors: {
                401: `Unauthorized`,
                403: `API-key auth used; JWT required`,
                404: `API key not found`,
            },
        });
    }
}
