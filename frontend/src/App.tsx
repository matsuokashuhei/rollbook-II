import { useQuery } from "@apollo/client";
import { AppBar, Box, Container, CssBaseline, Toolbar } from "@mui/material";
import { createTheme, ThemeProvider } from "@mui/material/styles";
import { useState } from "react";
import { graphql } from "../src/gql";
import "./App.css";

const theme = createTheme();

const allCourses = graphql(/* GraphQL */ `
  query GetCourses {
    schools {
      studios {
        timeSlots {
          dayOfWeek
          startTime
          course {
            id
            name
            price
            instructor {
              name
            }
          }
        }
      }
    }
  }
`);

function App() {
  const { data } = useQuery(allCourses, { variables: {} });
  const [count, setCount] = useState(0);

  return (
    <ThemeProvider theme={theme}>
      <Box>
        <CssBaseline />
        <AppBar position="static">
          <Toolbar variant="dense"></Toolbar>
        </AppBar>
        <Container maxWidth="xl">
          {/* {data && data.schools.map((school: any) => (
            <div key={school.id}></div>
          ))} */}
        </Container>
      </Box>
    </ThemeProvider>
  );
}

export default App;
