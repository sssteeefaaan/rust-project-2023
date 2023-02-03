<h1>Projekat iz predmeta <em>Paralelne i distribuirane arhitekture i jezici</em></h1>

<h2>Pokretanje</h2>
<ol>
  <li>Pozicionirati se u okviru direktorijuma <strong>lavirint</strong><br/> <code>cd ~/rust-project-2023/lavirint</code></li>
  <li>Izvršiti jednu od komandi:
    <ul>
      <li>Tipično pokretanje: <code>cargo run</code></li>
      <li>Brzo pokretanje: <code>cargo run --features bevy/dynamic</code> (Zahteva dodatne pakete izdvojene u sekciji Napomene)</li>
    </ul>
  </li>
</ol>

<h2>Napomene</h2>
<ul>
  <li>U okviru fajla <strong><em>primer.txt</em></strong> je moguće specificirati lavirint u formatu opisanom u okviru fajla <em>Projektna specifikacija.pdf</em>. Ovaj fajl prihvata samo nule(0) i jedinice(1) na osnovu kojih dalje vrši konverziju u binarni format iz koga se parsira lavirint.</li>

  <li>Projekat se oslanja na <a href="https://bevyengine.org/">Bevy alat</a> za rad sa grafičkim interfejsom, tako da je ovu biblioteku neophodno prevući pri pokretanju.</li>

  <li>Neophodno je imati instalirane pakete:
    <ul>
      <li><code>rust-alsa-sys-devel</code> dostupno na linkovima za linux distribucije: <a href="https://pkgs.org/download/rust-alsa-sys-devel">fedora</a> i <a href="https://packages.ubuntu.com/focal/librust-alsa-sys-dev">ubuntu</a></li>
      <li><code>rust-libudev-devel</code> neophodno za fedora distribuciju linux-a, dostupno na <a href="https://fedora.pkgs.org/35/fedora-x86_64/rust-libudev-sys+default-devel-0.1.4-12.fc35.noarch.rpm.html">linku</a></li>
    </ul>
    kako bi Bevy brzo pokretanje funkcionisalo (<code>cargo run</code> će raditi i bez ovih alata, samo će biti sporije pokretanje)</li>

</ul>
