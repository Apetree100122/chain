(function() {var implementors = {};
implementors["pallet_staking"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_staking/pallet/trait.Config.html\" title=\"trait pallet_staking::pallet::Config\">Config</a>&gt; OnOffenceHandler&lt;&lt;T as Config&gt;::AccountId, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.62.0/std/primitive.tuple.html\">(</a>&lt;T as Config&gt;::ValidatorId, &lt;T as Config&gt;::FullIdentification<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.62.0/std/primitive.tuple.html\">)</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.62.0/std/primitive.u64.html\">u64</a>&gt; for <a class=\"struct\" href=\"pallet_staking/pallet/struct.Pallet.html\" title=\"struct pallet_staking::pallet::Pallet\">Pallet</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Config&lt;ValidatorId = &lt;T as Config&gt;::AccountId&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Config&lt;FullIdentification = <a class=\"struct\" href=\"pallet_staking/pallet/struct.ValidatorSnapshot.html\" title=\"struct pallet_staking::pallet::ValidatorSnapshot\">ValidatorSnapshot</a>&lt;&lt;T as Config&gt;::AccountId, &lt;&lt;T as <a class=\"trait\" href=\"pallet_staking/pallet/trait.Config.html\" title=\"trait pallet_staking::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"pallet_staking/pallet/trait.Config.html#associatedtype.Currency\" title=\"type pallet_staking::pallet::Config::Currency\">Currency</a> as Currency&lt;&lt;T as Config&gt;::AccountId&gt;&gt;::Balance&gt;, FullIdentificationOf = <a class=\"struct\" href=\"pallet_staking/pallet/struct.ValidatorSnapshotOf.html\" title=\"struct pallet_staking::pallet::ValidatorSnapshotOf\">ValidatorSnapshotOf</a>&lt;T&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::SessionHandler: SessionHandler&lt;&lt;T as Config&gt;::AccountId&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::SessionManager: SessionManager&lt;&lt;T as Config&gt;::AccountId&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::ValidatorIdOf: Convert&lt;&lt;T as Config&gt;::AccountId, <a class=\"enum\" href=\"https://doc.rust-lang.org/1.62.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&lt;T as Config&gt;::AccountId&gt;&gt;,&nbsp;</span>","synthetic":false,"types":["pallet_staking::pallet::Pallet"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()