export default function Jobs() {
  return (
    <>
      <body class="bg-gray-100">
        <header class="bg-blue-500 text-white p-4 text-center">
          <h1 class="text-2xl font-semibold">BOUNTIES</h1>
        </header>

        <div class="container mx-auto grid gap-4 p-4 grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-3">
          <section class="bg-white p-4 rounded-md shadow-md">
            <h2 class="text-xl font-semibold mb-4">Section 1</h2>
            <p>Section 1 Content Goes Here</p>
          </section>

          <section class="bg-white p-4 rounded-md shadow-md">
            <h2 class="text-xl font-semibold mb-4">Section 2</h2>
            <p>Section 2 Content Goes Here</p>
          </section>

          <section class="bg-white p-4 rounded-md shadow-md">
            <h2 class="text-xl font-semibold mb-4">Section 3</h2>
            <p>Section 3 Content Goes Here</p>
          </section>
        </div>
      </body>
    </>
  );
}
