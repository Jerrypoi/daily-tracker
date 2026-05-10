/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type CreateDailyTrackRequest = {
    /**
     * Start time of the period (must be at :00 or :30)
     */
    start_time: string;
    /**
     * ID of the topic for this time period
     */
    topic_id: number;
    /**
     * Optional notes or comments
     */
    comment?: string;
    /**
     * Activity length in minutes; must be a positive multiple of 30 and at most 1440 (24 hours). Tracks for the same user may not overlap.
     */
    duration_minutes: number;
};

