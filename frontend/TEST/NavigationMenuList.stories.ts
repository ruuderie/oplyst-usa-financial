import type { Meta, StoryObj } from '@storybook/vue3';

import NavigationMenuList from '../components/ui/navigation-menu/NavigationMenuList.vue';

const meta = {
  title: 'NavigationMenuList',
  component: NavigationMenuList,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof NavigationMenuList>;

export default meta;
type Story = StoryObj<typeof NavigationMenuList>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};