/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type CreateApiKeyResponse = {
    id: number;
    name: string;
    key_prefix: string;
    /**
     * Plaintext API key. Only returned at creation time — store it now; it cannot be retrieved again.
     */
    token: string;
    created_at: string;
};

