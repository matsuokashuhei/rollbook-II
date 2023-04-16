// import { useQuery } from "@apollo/client";
import { RouterProvider } from "react-router-dom";
import router from "./router";
// import { graphql } from "../src/gql";

// const theme = createTheme();

// const allCourses = graphql(/* GraphQL */ `
//   query GetCourses {
//     schools {
//       studios {
//         timeSlots {
//           dayOfWeek
//           startTime
//           course {
//             id
//             name
//             price
//             instructor {
//               name
//             }
//           }
//         }
//       }
//     }
//   }
// `);

function App() {
  // const { loading, error, data } = useQuery(allCourses, { variables: {} });

  return (
    // <ThemeProvider theme={theme}>
    //   <Box>
    //     <CssBaseline />
    //     <AppBar position="static">
    //       <Toolbar variant="dense"></Toolbar>
    //     </AppBar>
    //     <Container maxWidth="xl"></Container>
    //   </Box>
    // </ThemeProvider>
    <RouterProvider router={router} />
  );
}

export default App;
