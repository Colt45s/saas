const Page = ({ params }: { params: { slug: string } }) => {
  return (
    <main>
      <h1>Project #{params.slug}</h1>
    </main>
  );
};

export default Page;
