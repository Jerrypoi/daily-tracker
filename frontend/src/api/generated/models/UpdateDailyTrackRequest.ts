/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type UpdateDailyTrackRequest = {
    /**
     * ID of the topic associated with this time period.
     */
    topic_id: string;
    /**
     * Optional comment for the time period.
     */
    comment?: string;
    /**
     * Activity length in minutes; must be a positive multiple of 30 and at most 1440 (24 hours). Tracks for the same user may not overlap.
     */
    duration_minutes: number;
};

