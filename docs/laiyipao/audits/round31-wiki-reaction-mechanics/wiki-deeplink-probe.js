const rs = Array.from(document.querySelectorAll('a')).filter(a => a.textContent.includes('评分') || a.getAttribute('href') === '#rating');
console.log(JSON.stringify({navCount: rs.length, hrefs: rs.map(a => a.getAttribute('href')), hashBefore: location.hash}));
if (rs.length) rs[0].click();
setTimeout(() => console.log(JSON.stringify({hashAfter: location.hash, scrollY: window.scrollY, targetY: Math.round(document.getElementById('rating')?.getBoundingClientRect().top ?? -1)}), 100);