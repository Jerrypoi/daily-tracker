/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CreateDailyTrackRequest } from '../models/CreateDailyTrackRequest';
import type { DailyTrack } from '../models/DailyTrack';
import type { UpdateDailyTrackRequest } from '../models/UpdateDailyTrackRequest';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class DailyTrackService {
    /**
     * Get daily track records
     * Retrieves a list of daily track records. Supports filtering by date range and topic.
     * @param startDate Filter records starting from this date (inclusive). Format: YYYY-MM-DD
     * @param endDate Filter records up to this date (inclusive). Format: YYYY-MM-DD
     * @param topicId Filter records by topic ID
     * @returns DailyTrack Successful operation
     * @throws ApiError
     */
    public static getDailyTracks(
        startDate?: string,
        endDate?: string,
        topicId?: number,
    ): CancelablePromise<Array<DailyTrack>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/daily-tracks',
            query: {
                'start_date': startDate,
                'end_date': endDate,
                'topic_id': topicId,
            },
            errors: {
                400: `Invalid date format or parameters`,
                500: `Internal server error`,
            },
        });
    }
    /**
     * Create a new daily track record
     * Creates a new daily track record. The start_time must be at :00 or :30 minutes of an hour.
     * @param body Daily track record to be created
     * @returns DailyTrack Daily track record created successfully
     * @throws ApiError
     */
    public static createDailyTrack(
        body: CreateDailyTrackRequest,
    ): CancelablePromise<DailyTrack> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/daily-tracks',
            body: body,
            errors: {
                400: `Invalid input (e.g., start_time not at :00 or :30)`,
                404: `Referenced topic not found`,
                409: `A record already exists for this time period`,
                500: `Internal server error`,
            },
        });
    }
    /**
     * Get a daily track record by ID
     * Retrieves a single daily track record by its ID.
     * @param id ID of the daily track record to retrieve
     * @returns DailyTrack Successful operation
     * @throws ApiError
     */
    public static getDailyTrackById(
        id: number,
    ): CancelablePromise<DailyTrack> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/daily-tracks/{id}',
            path: {
                'id': id,
            },
            errors: {
                404: `Daily track record not found`,
                500: `Internal server error`,
            },
        });
    }
    /**
     * Update a daily track record
     * Updates a daily track record's topic and comment by its ID.
     * @param id ID of the daily track record to update
     * @param body Daily track update payload
     * @returns DailyTrack Daily track updated successfully
     * @throws ApiError
     */
    public static updateDailyTrack(
        id: number,
        body: UpdateDailyTrackRequest,
    ): CancelablePromise<DailyTrack> {
        return __request(OpenAPI, {
            method: 'PUT',
            url: '/daily-tracks/{id}',
            path: {
                'id': id,
            },
            body: body,
            errors: {
                400: `Invalid input`,
                404: `Daily track or referenced topic not found`,
                500: `Internal server error`,
            },
        });
    }
    /**
     * Delete a daily track record
     * Deletes a daily track record by its ID.
     * @param id ID of the daily track record to delete
     * @returns void
     * @throws ApiError
     */
    public static deleteDailyTrack(
        id: number,
    ): CancelablePromise<void> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/daily-tracks/{id}',
            path: {
                'id': id,
            },
            errors: {
                404: `Daily track not found`,
                500: `Internal server error`,
            },
        });
    }
}
