/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { LoginRequest } from '../models/LoginRequest';
import type { RegisterRequest } from '../models/RegisterRequest';
import type { TokenResponse } from '../models/TokenResponse';
import type { UserResponse } from '../models/UserResponse';
import type { VerifyEmailRequest } from '../models/VerifyEmailRequest';
import type { VerifyEmailResponse } from '../models/VerifyEmailResponse';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class AuthService {
    /**
     * Register a new user
     * Registers a new user and returns user info.
     * @param body
     * @returns UserResponse Success
     * @throws ApiError
     */
    public static register(
        body: RegisterRequest,
    ): CancelablePromise<UserResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/auth/register',
            body: body,
        });
    }
    /**
     * Verify email address
     * Validates the verification code sent to the user's email. Must be called before login.
     * @param body
     * @returns VerifyEmailResponse Success
     * @throws ApiError
     */
    public static verifyEmail(
        body: VerifyEmailRequest,
    ): CancelablePromise<VerifyEmailResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/auth/verify-email',
            body: body,
            errors: {
                400: `Invalid or expired verification code`,
            },
        });
    }
    /**
     * Login and get a JWT token
     * Logs in the user and returns a token.
     * @param body
     * @returns TokenResponse Success
     * @throws ApiError
     */
    public static login(
        body: LoginRequest,
    ): CancelablePromise<TokenResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/auth/login',
            body: body,
        });
    }
}
