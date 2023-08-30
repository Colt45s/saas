/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  /**
   * Implement the DateTime<Utc> scalar
   *
   * The input/output is a string in RFC3339 format.
   */
  DateTime: { input: Date; output: Date; }
};

export type CreateUserInput = {
  email?: InputMaybe<Scalars['String']['input']>;
  emailVerified?: InputMaybe<Scalars['Boolean']['input']>;
  image?: InputMaybe<Scalars['String']['input']>;
  name?: InputMaybe<Scalars['String']['input']>;
  uid: Scalars['String']['input'];
};

export type MutationRoot = {
  __typename?: 'MutationRoot';
  createUser: User;
  deleteUser: User;
  hello: Scalars['String']['output'];
  signin: User;
  updateUser: User;
};


export type MutationRootCreateUserArgs = {
  input: CreateUserInput;
};


export type MutationRootDeleteUserArgs = {
  userId: Scalars['ID']['input'];
};


export type MutationRootSigninArgs = {
  token: Scalars['String']['input'];
};


export type MutationRootUpdateUserArgs = {
  input: UpdateUserInput;
};

export type Product = {
  __typename?: 'Product';
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['String']['output'];
  modifiedAt: Scalars['DateTime']['output'];
  name: Scalars['String']['output'];
  slug: Scalars['String']['output'];
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  hello: Scalars['String']['output'];
  user: User;
  viewer: User;
};


export type QueryRootUserArgs = {
  by: UserBy;
};

export type UpdateUserInput = {
  email?: InputMaybe<Scalars['String']['input']>;
  emailVerified?: InputMaybe<Scalars['Boolean']['input']>;
  id: Scalars['ID']['input'];
  image?: InputMaybe<Scalars['String']['input']>;
  name?: InputMaybe<Scalars['String']['input']>;
};

export type User = {
  __typename?: 'User';
  email: Maybe<Scalars['String']['output']>;
  emailVerified: Maybe<Scalars['Boolean']['output']>;
  id: Scalars['ID']['output'];
  image: Maybe<Scalars['String']['output']>;
  name: Maybe<Scalars['String']['output']>;
  projects: Array<Product>;
  uid: Scalars['String']['output'];
};

export type UserBy = {
  email?: InputMaybe<Scalars['String']['input']>;
  id?: InputMaybe<Scalars['ID']['input']>;
};

export type SigninMutationVariables = Exact<{
  token: Scalars['String']['input'];
}>;


export type SigninMutation = { __typename?: 'MutationRoot', signin: { __typename?: 'User', id: string, uid: string, name: string | null, email: string | null, emailVerified: boolean | null, image: string | null } };

export type ViewerQueryQueryVariables = Exact<{ [key: string]: never; }>;


export type ViewerQueryQuery = { __typename?: 'QueryRoot', viewer: { __typename?: 'User', id: string, name: string | null } };


export const SigninDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"Signin"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"token"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"signin"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"token"},"value":{"kind":"Variable","name":{"kind":"Name","value":"token"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"uid"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"email"}},{"kind":"Field","name":{"kind":"Name","value":"emailVerified"}},{"kind":"Field","name":{"kind":"Name","value":"image"}}]}}]}}]} as unknown as DocumentNode<SigninMutation, SigninMutationVariables>;
export const ViewerQueryDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"ViewerQuery"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"viewer"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}}]}}]}}]} as unknown as DocumentNode<ViewerQueryQuery, ViewerQueryQueryVariables>;