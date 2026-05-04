/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type ApiKey = {
    /**
     * Public identifier for the API key
     */
    id: number;
    /**
     * User-supplied label for the key
     */
    name: string;
    /**
     * First few characters of the token (e.g. 'dt_a1b2c3') for display only
     */
    key_prefix: string;
    created_at: string;
    /**
     * Last time this key successfully authenticated, or null if never used
     */
    last_used_at?: string;
};

