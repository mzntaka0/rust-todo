"use client";

import React from "react";
import { CircularProgress, Stack } from "@mui/material";
import { useAspidaQuery } from "@aspida/react-query";
import aspida from "@aspida/axios";
import aspidaFetch from "@aspida/fetch";

import { api as axiosApi } from "lib/fetcher";
import { tauriFetch } from "lib/tauri";
import api from "api/$api";
import TodoListItem from "./TodoListItem";

const Component: React.FC = () => {
  const client = api(aspida(axiosApi));
  //const tauriClient = api(aspidaFetch(tauriFetch));
  const { data: _todos } = useAspidaQuery(client.todos);
  //const { data: _tauriTodos } = useAspidaQuery(tauriClient.todos);

  if (!_todos) {
    return <CircularProgress />;
  }

  //console.log(_tauriTodos);
  const todos = _todos;
  console.log(todos);
  return (
    <Stack
      sx={{
        maxWidth: "800px",
        width: "100%",
        borderRadius: 2,
        overflow: "hidden",
      }}
    >
      {todos.map((value) => (
        <TodoListItem key={value.id} {...value} />
      ))}
    </Stack>
  );
};

export default Component;
