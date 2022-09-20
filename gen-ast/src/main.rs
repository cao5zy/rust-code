#[macro_use]
extern crate json;

extern crate getopts;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io::Write;

fn build_args() -> String {
    let parsed = json::parse(
        r#"
                             {
                             "i":{
                             "long": "input",
                             "desc": "需要解析的文件名称"
                             }
                             }

    "#,
    )
    .unwrap();

    for (key, value) in parsed.entries().rev(){
        println!("{}", key);
        println!("{}", value["long"]);
        println!("{}", value["desc"]);
    }
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();

    opts.optopt("i", "--input", "需要解析的文件名称", "input");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            panic!("{}", e)
        }
    };

    matches.opt_str("i").unwrap()
}

fn main() {
    use std::io::stderr;
    use swc::config::SourceMapsConfig;
    use swc::ecmascript::ast::ModuleItem;
    use swc::ecmascript::ast::{EsVersion, ImportDecl, ModuleDecl};
    use swc::Compiler;
    use swc_atoms::JsWord;
    use swc_common::sync::Lrc;
    use swc_common::{errors::Handler, FileName, SourceMap, DUMMY_SP};
    use swc_ecma_ast::{ImportSpecifier, Str};
    use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

    println!("{}", build_args());

    let source = "
import { Component } from '@tarojs/taro'
import { Text } from '@tarojs/components'
// eslint-disable-next-line import/extensions
import { env, yum, nav } from '@/taro-yum-common'
// eslint-disable-next-line no-unused-vars
import BasePage from '@/kfc/kids/components/base-page'
import { PopScreen } from '@/kfc/kids/components'
import ActivityShare from '@/kfc/kids/components/activity-share'
import { shouldShowShareLayer, h5ShareActivity, buildWxAppShareData, getShareData } from '@/kfc/kids/components/activity-share/shareHelper'
// import VideoBanner from './videoBanner'
import kidsDataManager from '@/kfc/kids/common/kidsDataManager'
// import activityDataManager from '../activityDataManager'
import { sizer, kidsEnv, checkIsNewActivity } from '@/kfc/kids/common/utils'
import ActivityCardList from '../others'
import BottomBar from '../bottomBar'
import ImageList from './imglist'
import Header from '../header'
import './index.less'
import Banner from '../banner'
import ReturnList from '../returnList'
/**
 * 活动详情 接口里定义为theme 以后的代码中出现的名词activity即为接口层的theme
 */
export default class ActivityDetail extends Component {
  pageTitle = '活动详情'

  config = {
    navigationBarTitleText: '活动详情',
    navigationStyle: 'custom',
    disableScroll: !env.isWeapp
  }

  constructor(props) {
    super(props)
    this.state = {
      // eslint-disable-next-line react/no-unused-state
      data: {},
      isNewActivity: false, // 是否从小红盒活动列表进入
      status: -1, // 小红盒活动状态
      showShare: false // 显示分享弹层
    }
  }

  componentDidMount() {
    // 2020030301  2020030302  2020030303  2020030304  2020030305
    // 测试
    const { themeId, status, activityType = '' } = this.$router.params || {}
    const newStatus = (status && JSON.parse(status)) || -1
    const isNewActivity = checkIsNewActivity(activityType)
    this.setState({ isNewActivity, status: newStatus })
    if (!themeId) {
      yum.showToast('请选择要参加的活动')
      return
    }

    if (env.isWeapp && wx && wx.showShareMenu) {
      wx.showShareMenu({
        withShareTicket: true,
        menus: ['shareAppMessage', 'shareTimeline']
      })
    }

    kidsDataManager
      .activityJson(themeId)
      .then((data) => {
        // eslint-disable-next-line react/no-unused-state
        this.setState({ data }, () => {
          if (env.isWebInWeapp) {
            this.share()
          }
        })
      })
      .catch((e) => {
        console.log(e)
      })
  }

  /**
   * weapp分享3+
   * .
   */
  onShareAppMessage() {
    const { data, status, activityType } = this.state
    return buildWxAppShareData({ ...getShareData(data), status, activityType })
  }

  share = () => {
    // app端手动分享
    const { data } = this.state
    h5ShareActivity(data)
  }

  back = () => {
    const { isNewActivity } = this.state
    if ((env.isWeb && !isNewActivity) || (env.isWeapp && nav.getCurrentPages().length <= 1)) {
      // weapp 该页可能来自分享入口 back回首页
      // h5 请求登录页无法回退的问题
      // h5 需要认为来自首页，强制返回首页(跳转登录的问题history已重置)
      kidsEnv.backToHome({ forceWebReplace: false, fromHome: true })
    } else {
      nav.navigateBack()
    }
  }
  // menuButtonClick = (e, name) => {
  //   if (name === 'share') {
  //     this.share()
  //   }
  // }

  scrollOffset = (x, y) => {
    if (this.$header) {
      this.$header.pageOffsetYChanged(y)
    }
  }

  previewImages = (index = 0, urls = []) => {
    // const urls = this.props.imgUrls || []
    yum.previewImage({
      rnDisableSaveButton: true,
      current: urls[index], // 当前显示图片的http链接
      urls // 需要预览的图片http链接列表
    })
  }

  showShareLayer = () => {
    if (shouldShowShareLayer()) this.setState({ showShare: true })
  }

  hideShareLayer = () => {
    this.setState({ showShare: false })
  }

  render() {
    const { data, isNewActivity, status, showShare } = this.state
    const { themeName = '', activityType, contentInfo = [], otherThemes = [], returnImg = [], activityImg = [], themeId } = data || {}
    const imgUrls = contentInfo
      ? contentInfo
          .sort((a, b) => a.sortRank - b.sortRank)
          .map((info) => {
            return info.imgSrc
          })
      : []
    // const menuButtons = [{ icon: shareIcon, name: 'share' }]
    const scrollViewHeightStyle = kidsEnv.scrollContentHeight()
    const returnImgList = returnImg && returnImg.length ? returnImg : activityImg
    return (
      <BasePage
        onScrollOffset={this.scrollOffset}
        style={{ backgroundColor: '#fff', ...scrollViewHeightStyle }}
        showHeader={env.isRn}
        // fakeHeaderStyle={{ height: sizer.pxSize(460) }}
        renderHeader={
          <Header
            onBack={this.back}
            ref={(ele) => {
              this.$header = ele
            }}
            onShare={this.showShareLayer}
            title={this.pageTitle}
          />
        }
        renderFooter={<BottomBar data={data} status={status} onShare={this.showShareLayer} />}
        fakeFooterStyle={{ 'minHeight': sizer.pxSize(136), width: '100%' }}
      >
        <Banner status={status} data={data} />
        {/* 不在在basePage中renderHeader,因为basePage中统一了fakeView显示,这里自定义控制 */}
        <Text className='actyd_title'>{themeName}</Text>
        <ImageList imgUrls={imgUrls} previewImages={this.previewImages} />
        {isNewActivity ? (
          <ReturnList isNewActivity returnImgList={returnImgList} status={status} themeId={themeId} previewImages={this.previewImages} />
        ) : null}
        <ActivityCardList items={otherThemes} activityType={activityType} />
        {!env.isRn ? (
          <Header
            onBack={this.back}
            ref={(ele) => {
              this.$header = ele
            }}
            onShare={this.showShareLayer}
            title={this.pageTitle}
          />
        ) : null}
        {showShare && (
          <PopScreen onCancel={this.hideShareLayer}>
            <ActivityShare shareItem={data} close={this.hideShareLayer} />
          </PopScreen>
        )}
      </BasePage>
    )
  }
}
    ";

    let _source2 = "\
        function abc(){\
            console.log(123);
        }";

    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Custom("test.js".into()), source.into());

    let compiler = Compiler::new(cm.clone());
    // let _handler = Handler::with_emitter_writer(Box::new(stderr()), Some(compiler.cm.clone()));

    let lexer = Lexer::new(
        // We want to parse ecmascript
        Syntax::Typescript(TsConfig {
            tsx: true,
            decorators: true,
            dynamic_import: true,
            dts: false,
            no_early_errors: false,
            import_assertions: false,
        }),
        // EsVersion defaults to es5
        EsVersion::Es2016,
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let list_error = parser.take_errors();
    if list_error.iter().len() > 0 {
        let mut err_msg = "".to_owned();
        for err in list_error {
            let msg = err.into_kind().msg().to_string();
            err_msg.push_str(msg.as_str());
        }
    }

    let mut module = parser.parse_module().unwrap();

    println!("parser success");

    let s = serde_json::to_string_pretty(&module).expect("failed to serialize");
    // println!("ast json is \n {}", s);
    let mut file = File::create("ast.json").expect("create failed");
    file.write_all(s.as_bytes()).expect("write failed");
}
