"use client";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import React, { FC } from "react";
import { useForm, SubmitHandler } from "react-hook-form";

interface IFormInput {
  firstName: string;
  lastName: string;
  email: string;
  password: string;
  confirmPassword: string;
}

/**
 * @author Bala
 * @function @Register
 **/
export default function Register() {
  const {
    register,
    formState: { errors },
    getValues,
    handleSubmit,
  } = useForm<IFormInput>();
  const onSubmit: SubmitHandler<IFormInput> = (data) => console.log(data);
  return (
    <form className="w-4/12 p-10 space-y-4 dark:border-grey-700 shadow rounded-lg  md:mt-0 ">
      <h1 className="ext-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl dark:text-white">
        Create an account
      </h1>
      <div className="space-y-2">
        <label htmlFor="firstName">First Name</label>
        <Input
          {...register("firstName", { required: "First Name is required" })}
          id="firstName"
          aria-invalid={errors.firstName ? "true" : "false"}
        />
        {errors.firstName?.type === "required" && (
          <p className="text-red-950">First name is required</p>
        )}
      </div>
      <div className="space-y-2">
        <label htmlFor="lastName">Last Name</label>
        <Input {...register("lastName", { required: true })} id="lastName" />
        {errors.lastName?.type === "required" && (
          <p className="text-red-950">Last name is required</p>
        )}
      </div>
      <div className="space-y-2">
        <label htmlFor="email">Email</label>
        <Input
          {...register("email", {
            required: "Email is required",
            pattern: {
              value:
                /^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/,
              message: "Invalid email address",
            },
          })}
          id="email"
          type="email"
        />
        {errors.email && <p className="text-red-950">{errors.email.message}</p>}
      </div>
      <div className="space-y-2">
        <label htmlFor="password">Password</label>
        <Input
          {...register("password", {
            required: "Password is required",
            minLength: {
              value: 8,
              message: "Minimum length should be 8 characters",
            },
          })}
          id="password"
          type="password"
        />
        {errors.password && (
          <p className="text-red-950">{errors.password?.message}</p>
        )}
      </div>
      <div className="space-y-2">
        <label htmlFor="confirmPassword">Confirm Password</label>
        <Input
          {...register("confirmPassword", {
            required: true,
            validate: {
              matchesPreviousPassword: (value) => {
                const { password } = getValues();
                return password === value || "Passwords should match!";
              },
            },
          })}
          id="confirmPassword"
          type="password"
        />
        {errors.confirmPassword && (
          <p className="text-red-950">{errors.confirmPassword.message}</p>
        )}
      </div>
      <Button
        variant="default"
        size="register"
        onClick={handleSubmit(onSubmit)}
        className="margin-5"
      >
        Submit
      </Button>
      <div>
        <p className="text-sm font-dark text-gray-900 ">
          Already have an account? &nbsp;
          <a
            href="#"
            className="text-primary-600 hover:underline dark:text-primary-500"
          >
            Login here
          </a>
        </p>
      </div>
    </form>
  );
}
