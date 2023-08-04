export default function Home() {
  return (
    <div className="bg-base-100 shadow-xl p-8 max-w-2xl mx-auto rounded-box">
      <h2 className="text-3xl mb-4">What is Ferrite Analytics?</h2>
      <p className="mb-2">
        Ferrite Analytics is an open source analytics solution that respects
        users privacy. It typically tracks information such as page views,
        buttons clicked, and other interactions in websites and apps; without
        recording any private or personally identifiable information.
      </p>
      <p className="mb-2">
        Ferrite Analytics is a self-hosted solution. This page you are seeing is
        hosted by someone (maybe you!) on their own server. If you have any
        questions or concerns, please direct them to the owner of the website.
        Ferrite Analytics contributors do not own, control, or have access to
        this deployment of Ferrite Analytics (unless this is the demo instance).
      </p>
    </div>
  );
}
