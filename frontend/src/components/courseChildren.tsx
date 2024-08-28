const getCourseChildren = async (searchData: any, setLoading: any) => {
    setLoading(true);
    return await fetch("/api/generate/getCourseOpts", {
        method: "POST",
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(searchData),
    }).then((res) => {
        if (res.status === 200) {
            return res.json();
        } else {
            throw new Error("Unable to get course children");
        }
    }).then((data) => {
        setLoading(false);
        console.log(JSON.stringify(data));
        return data;
    }).catch((err) => {
        setLoading(false);
        return [];
    });
}

export default getCourseChildren;
