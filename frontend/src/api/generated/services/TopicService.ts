/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CreateTopicRequest } from '../models/CreateTopicRequest';
import type { Topic } from '../models/Topic';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class TopicService {
    /**
     * Get all topics
     * Retrieves a list of all topics. Supports optional filtering by parent_topic_id.
     * @param parentTopicId Filter topics by parent topic ID.
     * @returns Topic Successful operation
     * @throws ApiError
     */
    public static getTopics(
        parentTopicId?: number,
    ): CancelablePromise<Array<Topic>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/topics',
            query: {
                'parent_topic_id': parentTopicId,
            },
            errors: {
                500: `Internal server error`,
            },
        });
    }
    /**
     * Create a new topic
     * Creates a new topic with the given name and optional parent topic.
     * @param body Topic object to be created
     * @returns Topic Topic created successfully
     * @throws ApiError
     */
    public static createTopic(
        body: CreateTopicRequest,
    ): CancelablePromise<Topic> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/topics',
            body: body,
            errors: {
                400: `Invalid input`,
                409: `Topic name already exists`,
                500: `Internal server error`,
            },
        });
    }
    /**
     * Get a topic by ID
     * Retrieves a single topic by its ID.
     * @param id ID of the topic to retrieve
     * @returns Topic Successful operation
     * @throws ApiError
     */
    public static getTopicById(
        id: number,
    ): CancelablePromise<Topic> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/topics/{id}',
            path: {
                'id': id,
            },
            errors: {
                404: `Topic not found`,
                500: `Internal server error`,
            },
        });
    }
}
