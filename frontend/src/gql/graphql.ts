/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  /**
   * ISO 8601 calendar date without timezone.
   * Format: %Y-%m-%d
   *
   * # Examples
   *
   * * `1994-11-13`
   * * `2000-02-24`
   */
  NaiveDate: any;
  /**
   * ISO 8601 combined date and time without timezone.
   *
   * # Examples
   *
   * * `2015-07-01T08:59:60.123`,
   */
  NaiveDateTime: any;
};

export type Attendance = {
  __typename?: 'Attendance';
  createdAt: Scalars['NaiveDateTime'];
  id: Scalars['Int'];
  lessonId: Scalars['Int'];
  member: Member;
  memberId: Scalars['Int'];
  updatedAt: Scalars['NaiveDateTime'];
};

export type Course = {
  __typename?: 'Course';
  courseSchedules: Array<CourseSchedule>;
  createdAt: Scalars['NaiveDateTime'];
  endDate: Scalars['NaiveDate'];
  id: Scalars['Int'];
  instructor: Instructor;
  instructorId: Scalars['Int'];
  lessons: Array<Lesson>;
  members: Array<Member>;
  name: Scalars['String'];
  price: Scalars['Int'];
  startDate: Scalars['NaiveDate'];
  studio: Studio;
  studioId: Scalars['Int'];
  timeSlot: TimeSlot;
  updatedAt: Scalars['NaiveDateTime'];
};

export type CourseSchedule = {
  __typename?: 'CourseSchedule';
  course: Course;
  courseId: Scalars['Int'];
  createdAt: Scalars['NaiveDateTime'];
  endDate: Scalars['NaiveDate'];
  id: Scalars['Int'];
  startDate: Scalars['NaiveDate'];
  timeSlot: TimeSlot;
  timeSlotId: Scalars['Int'];
  updatedAt: Scalars['NaiveDateTime'];
};

export type CreateAttendanceInput = {
  lessonId: Scalars['Int'];
  memberId: Scalars['Int'];
};

export type CreateCourseInput = {
  endDate: Scalars['NaiveDate'];
  instructorId: Scalars['Int'];
  name: Scalars['String'];
  price: Scalars['Int'];
  startDate: Scalars['NaiveDate'];
  studioId: Scalars['Int'];
};

export type CreateInstructorInput = {
  email: Scalars['String'];
  name: Scalars['String'];
  phoneNumber: Scalars['String'];
};

export type CreateLessonInput = {
  courseScheduleId: Scalars['Int'];
  date: Scalars['NaiveDate'];
};

export type CreateMemberInput = {
  kana: Scalars['String'];
  name: Scalars['String'];
  number: Scalars['Int'];
};

export type CreateMembersCourseInput = {
  courseId: Scalars['Int'];
  endDate: Scalars['NaiveDate'];
  memberId: Scalars['Int'];
  startDate: Scalars['NaiveDate'];
};

export type CreateSchoolInput = {
  name: Scalars['String'];
};

export type CreateStudioInput = {
  name: Scalars['String'];
  schoolId: Scalars['Int'];
};

export enum DayOfWeek {
  Friday = 'FRIDAY',
  Monday = 'MONDAY',
  Saturday = 'SATURDAY',
  Sunday = 'SUNDAY',
  Thursday = 'THURSDAY',
  Tuesday = 'TUESDAY',
  Wednesday = 'WEDNESDAY'
}

export type Instructor = {
  __typename?: 'Instructor';
  courses: Array<Course>;
  createdAt: Scalars['NaiveDateTime'];
  email: Scalars['String'];
  id: Scalars['Int'];
  name: Scalars['String'];
  phoneNumber: Scalars['String'];
  updatedAt: Scalars['NaiveDateTime'];
};

export type Lesson = {
  __typename?: 'Lesson';
  courseSchedule: CourseSchedule;
  courseScheduleId: Scalars['Int'];
  date: Scalars['NaiveDate'];
  id: Scalars['Int'];
  members: Array<Member>;
  timeSlot: TimeSlot;
};

export type Member = {
  __typename?: 'Member';
  courses: Array<Course>;
  createdAt: Scalars['NaiveDateTime'];
  id: Scalars['Int'];
  kana: Scalars['String'];
  lessons: Array<Lesson>;
  membersCourses: Array<MembersCourse>;
  name: Scalars['String'];
  number: Scalars['Int'];
  updatedAt: Scalars['NaiveDateTime'];
};

export type MembersCourse = {
  __typename?: 'MembersCourse';
  course: Course;
  courseId: Scalars['Int'];
  createdAt: Scalars['NaiveDateTime'];
  endDate: Scalars['NaiveDate'];
  id: Scalars['Int'];
  member: Member;
  memberId: Scalars['Int'];
  startDate: Scalars['NaiveDate'];
  updatedAt: Scalars['NaiveDateTime'];
};

export type Mutation = {
  __typename?: 'Mutation';
  createAttandance: Attendance;
  createCourse: Course;
  createInstructor: Instructor;
  createLesson: Lesson;
  createMember: Member;
  createMembersCourse: MembersCourse;
  createSchool: School;
  createStudio: Studio;
};


export type MutationCreateAttandanceArgs = {
  input: CreateAttendanceInput;
};


export type MutationCreateCourseArgs = {
  input: CreateCourseInput;
};


export type MutationCreateInstructorArgs = {
  input: CreateInstructorInput;
};


export type MutationCreateLessonArgs = {
  input: CreateLessonInput;
};


export type MutationCreateMemberArgs = {
  input: CreateMemberInput;
};


export type MutationCreateMembersCourseArgs = {
  input: CreateMembersCourseInput;
};


export type MutationCreateSchoolArgs = {
  input: CreateSchoolInput;
};


export type MutationCreateStudioArgs = {
  input: CreateStudioInput;
};

export type Query = {
  __typename?: 'Query';
  instructors: Array<Instructor>;
  members: Array<Member>;
  schools: Array<School>;
};

export type School = {
  __typename?: 'School';
  createdAt: Scalars['NaiveDateTime'];
  id: Scalars['Int'];
  name: Scalars['String'];
  studios: Array<Studio>;
  updatedAt: Scalars['NaiveDateTime'];
};

export type Studio = {
  __typename?: 'Studio';
  courses: Array<Course>;
  createdAt: Scalars['NaiveDateTime'];
  id: Scalars['Int'];
  name: Scalars['String'];
  school: School;
  schoolId: Scalars['Int'];
  timeSlots: Array<TimeSlot>;
  updatedAt: Scalars['NaiveDateTime'];
};

export type TimeSlot = {
  __typename?: 'TimeSlot';
  course?: Maybe<Course>;
  courseSchedules: Array<CourseSchedule>;
  courses: Array<Course>;
  createdAt: Scalars['NaiveDateTime'];
  dayOfWeek: DayOfWeek;
  endTime: Scalars['String'];
  id: Scalars['Int'];
  startTime: Scalars['String'];
  studio: Studio;
  studioId: Scalars['Int'];
  updatedAt: Scalars['NaiveDateTime'];
};

export type GetCoursesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetCoursesQuery = { __typename?: 'Query', schools: Array<{ __typename?: 'School', studios: Array<{ __typename?: 'Studio', timeSlots: Array<{ __typename?: 'TimeSlot', dayOfWeek: DayOfWeek, startTime: string, course?: { __typename?: 'Course', id: number, name: string, price: number, instructor: { __typename?: 'Instructor', name: string } } | null }> }> }> };


export const GetCoursesDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"GetCourses"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"schools"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"studios"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"timeSlots"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"dayOfWeek"}},{"kind":"Field","name":{"kind":"Name","value":"startTime"}},{"kind":"Field","name":{"kind":"Name","value":"course"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"price"}},{"kind":"Field","name":{"kind":"Name","value":"instructor"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]}}]}}]}}]}}]} as unknown as DocumentNode<GetCoursesQuery, GetCoursesQueryVariables>;