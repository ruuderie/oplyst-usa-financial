import type { Meta, StoryObj } from '@storybook/vue3';

import NavigationMenu from '../components/ui/navigation-menu/NavigationMenu.vue';

const meta = {
  title: 'NavigationMenu',
  component: NavigationMenu,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof NavigationMenu>;

export default meta;
type Story = StoryObj<typeof NavigationMenu>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};